use self::config::TemplateConfig;
use crate::{
  cli::{Cli, Config},
  error::{Error, ErrorKind, Result},
};

use walkdir::WalkDir;

use std::path::{Path, PathBuf};

pub(crate) mod config;
#[cfg(feature = "git")]
#[cfg(feature = "hbs")]
pub mod git;
#[cfg(feature = "hbs")]
pub mod guidon;
#[cfg(feature = "hbs")]
pub(crate) mod helpers;

/// Template information.
pub struct Template {
  /// Name of project.
  pub name: String,
  /// Template source path.
  pub path: PathBuf,
  /// Template configuration.
  pub config: TemplateConfig,
}

impl Template {
  /// Create a new template instace.
  fn new(
    name: &str,
    path: &dyn AsRef<Path>,
    config: TemplateConfig,
  ) -> Result<Template> {
    // Template src path.
    let path = path.as_ref().to_owned();

    if !path.is_dir() {
      return Err(Error::new(
        ErrorKind::NotADirectory,
        &format!("{} is not a project directory.", path.display()),
      ));
    }

    Ok(Template {
      name: name.to_string(),
      path,
      config,
    })
  }
}

impl From<Config> for Template {
  fn from(c: Config) -> Template {
    match Self::new(&c.name, &c.path, TemplateConfig::new(&c.path)) {
      Ok(template) => template,
      Err(err) => panic!("{}", err),
    }
  }
}

impl From<Cli<'_>> for Template {
  fn from(c: Cli) -> Template {
    Self::from(c.config)
  }
}

impl Template {
  pub fn generate(&self, dest: &dyn AsRef<Path>) -> Result<()> {
    // Target destination where template will be created.
    let target: PathBuf = dest.as_ref().to_owned();

    // Create destination folders.
    std::fs::create_dir_all(dest.as_ref())?;

    // Walk the path and copy src path over to dest path.
    for entry in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
      if entry.path().is_dir() {
        std::fs::create_dir_all(entry.path())?;
        continue;
      } else if entry.path().is_file() {
        // Open the file.
        std::fs::copy(entry.path(), &target)?;
      } else {
        eprintln!("Do not know what's happening here...");
      }
      println!("{}", &entry.path().display());
    }
    Ok(())
  }
}
