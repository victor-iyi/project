use crate::{
  error::{Error, Result},
  git::GitOptions,
  template::config::TemplateConfig,
  util::basename,
};

use heck::{KebabCase, SnakeCase};
use url::{ParseError, Url};

use std::{
  env, fs,
  path::{Path, PathBuf},
};

/// Information about the new project to be created.
#[derive(Debug)]
pub(crate) struct ProjectInfo {
  /// The project name, which is extracted from the project's base directory.
  pub(crate) name: String,
  /// Base directory of the target project.
  pub(crate) path: PathBuf,
}

impl ProjectInfo {
  /// Create a new project info: given project local path.
  pub(crate) fn new(s: &dyn AsRef<Path>) -> Self {
    Self::from(s)
  }
}

impl ProjectInfo {
  /// Get the raw project name.
  pub(crate) fn raw(&self) -> String {
    self.name.to_owned()
  }

  /// Get the project name in Kebab case.
  pub(crate) fn name_kebab_case(&self) -> String {
    self.name.to_kebab_case()
  }

  /// Get the project name in snake case.
  pub(crate) fn name_snake_case(&self) -> String {
    self.name.to_snake_case()
  }

  /// Get owned project path.
  pub(crate) fn path(&self) -> PathBuf {
    self.path.clone()
  }

  pub(crate) fn path_kebab_case(&self) -> String {
    self.path.to_str().unwrap().to_kebab_case()
  }
}

impl From<&str> for ProjectInfo {
  fn from(s: &str) -> Self {
    Self {
      name: basename(s).into(),
      path: PathBuf::from(s),
    }
  }
}

impl From<&dyn AsRef<Path>> for ProjectInfo {
  fn from(p: &dyn AsRef<Path>) -> Self {
    Self {
      name: basename(p.as_ref().to_str().unwrap()).into(),
      path: PathBuf::from(p.as_ref()),
    }
  }
}

impl Default for ProjectInfo {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      path: env::current_dir().unwrap_or_default(),
    }
  }
}

/// `TemplateOptions` describes the kind of template we are using,
/// either a remote template or a local template.
pub(crate) enum TemplateOptions {
  /// A local template with the path to the base template directory.
  Local(PathBuf),

  /// A remote template with the URI to the remote repo and other
  /// git options (e.g) branch stating how we want to fetch the remote
  /// template.
  Remote(GitOptions),
}

impl TemplateOptions {
  /// Creates a `TemplateOption` given a file path or URL. URL can either be a full
  /// git URL e.g https://github.com/username/repo  or a shortened form e.g
  /// `username/repo`. It can also be an absolute or relative path.
  ///
  /// Note that relative file path e.g `../../some/path/` will be resolved into it's
  /// full absolute path.
  ///
  /// `branch` represents the branch to checkout if it's a git repo.
  pub(crate) fn new(path: &str, branch: Option<String>) -> TemplateOptions {
    // https://github.com/username/repo
    // username/repo
    // relative/path/to/template
    match Self::parse_path(path, branch) {
      Ok(opts) => opts,
      Err(err) => panic!("Error: {}", err),
    }
  }

  /// Parses a given path as URL or local file path.
  ///
  /// Path can be one of:
  /// - A Full URL e.g. https://github.com/username/repo
  /// - A Shortened Git repo e.g. username/repo
  /// - A local file path.
  fn parse_path(path: &str, branch: Option<String>) -> Result<Self> {
    let opts = match Url::parse(path) {
      // A valid URL. -- Remote
      Ok(url) => Self::Remote(GitOptions::new(url, branch)),
      Err(ParseError::RelativeUrlWithoutBase) => {
        // Might be a relative path or a shortened Git URI.
        match fs::canonicalize(path) {
          // Relative local file path.
          Ok(p) => Self::Local(p),
          Err(err) => {
            eprintln!("does not exist. {}", err);
            // Short Git URI.
            Self::parse_path(&format!("https://github.com/{}", path), branch)?
          }
        }
      }
      Err(err) => {
        return Err(Error::from(err));
      }
    };

    Ok(opts)
  }
}

impl From<&dyn AsRef<Path>> for TemplateOptions {
  fn from(path: &dyn AsRef<Path>) -> TemplateOptions {
    TemplateOptions::new(path.as_ref().to_str().unwrap(), None)
  }
}

impl Default for TemplateOptions {
  fn default() -> Self {
    Self::Local(env::current_dir().unwrap_or_default())
  }
}

/// Command line argument configuration.
pub(crate) struct TemplateInfo {
  /// The kind of template to be used (local or remote).
  options: TemplateOptions,

  /// Template configuration file.
  config: TemplateConfig,
}

impl TemplateInfo {
  pub(crate) fn new(path: &str, branch: Option<&str>) -> Self {
    let opts = TemplateOptions::new(path, branch.map(|s| s.to_string()));
    match opts {
      TemplateOptions::Local(path) => {
        //
        println!("{}", path.display());
      }
      TemplateOptions::Remote(git_opts) => {
        println!("Download repo to a temp file");
      }
    }

    Self::default()
  }
}

impl Default for TemplateInfo {
  fn default() -> TemplateInfo {
    TemplateInfo {
      options: TemplateOptions::default(),
      config: TemplateConfig::default(),
    }
  }
}
