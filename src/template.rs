use walkdir::{DirEntry, WalkDir};

use crate::{
  cli::{Arguments, Cli},
  error::Result,
  git::{self, GitOptions},
  info::{ProjectInfo, TemplateOptions},
  template::config::TemplateConfig,
};

use std::{
  fs::{self, File},
  io::{BufReader, Read, Write},
  path::Path,
};

pub(crate) mod config;
#[cfg(feature = "git")]
// #[cfg(feature = "hbs")]
// pub mod git;
#[cfg(feature = "hbs")]
pub mod guidon;
#[cfg(feature = "hbs")]
pub(crate) mod helpers;
pub(crate) mod parser;

/// Template builds and generates the project from a given template.
pub struct Template {
  template: TemplateMeta,
}

impl Template {
  pub fn new(
    project_info: &ProjectInfo,
    template_options: &TemplateOptions,
  ) -> Template {
    Template {
      template: TemplateMeta::new(project_info, template_options),
    }
  }
}

impl Template {
  pub fn generate(&self) -> Result<()> {
    // Project path.
    let project_dir = self.template.project_info.path.as_path();
    // Template path.
    let template_dir = self.template.template_options.path();

    // Walk the `template_dir`.
    for entry in WalkDir::new(template_dir)
      .into_iter()
      .filter_entry(|e| !self.filter_ignore(e))
      .filter_map(|e| e.ok())
    {
      // Strip `template_dir` from entry.
      let relative_path = entry.path().strip_prefix(template_dir)?;
      // Append stripped path to `project_dir`.
      let target = project_dir.join(relative_path);

      // TODO: Check configuration for path (`target`) to rename.
      // for var in &self.template.config.rename {
      //   for key in var.keys() {
      //     if let Some(value) = var.get(key) {
      //       // Rename key in `target` to `val`.
      //     }
      //   }
      // }

      if entry.path().is_dir() {
        fs::create_dir_all(&target)?;
        continue;
      } else {
        // TODO: Check for files eligible for variable substitution.
        if entry.path().ends_with("hbs") {
          println!(
            "Performing template substitution for {}.",
            entry.path().display()
          );

          // TODO: Perform template substitution.
          let content = self.substitue(&entry.path())?;

          // Writing substituted content into `target`.
          let mut file = File::create(&target)?;
          file.write_all(&content.as_bytes())?;
        } else {
          // Copy over files.
          fs::copy(entry.path(), &target)?;
        }
      }
    }

    println!("Done generating template into {}", project_dir.display());
    Ok(())
  }

  fn substitue(&self, entry: &Path) -> Result<String> {
    // Open file for reading.
    let file = File::open(entry)?;
    let mut buf_reader = BufReader::new(file);

    // Read file contents into `contents`.
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // TODO: Do some substitution with `contents`.
    Ok(contents)
  }

  fn filter_ignore(&self, entry: &DirEntry) -> bool {
    // TODO: Filter ignored/included files here...
    entry
      .file_name()
      .to_str()
      .map(|s| {
        s.contains("venv") || s.contains(".vscode") || s.contains(".DS_Store")
      })
      .unwrap_or(false)
  }
}

impl From<&Arguments> for Template {
  fn from(args: &Arguments) -> Template {
    Template {
      template: TemplateMeta::new(&args.project, &args.template),
    }
  }
}

impl From<&Cli<'_>> for Template {
  fn from(cli: &Cli) -> Template {
    Self::from(&cli.args)
  }
}

impl Default for Template {
  fn default() -> Template {
    Template {
      template: TemplateMeta::default(),
    }
  }
}

/// Template & project comes together to load the template from remote or local
/// path, loads the `"template.toml"` config file, and initializes git for the
/// new project.
struct TemplateMeta {
  #[doc(hidden)]
  template_options: TemplateOptions,

  #[doc(hidden)]
  config: TemplateConfig,

  #[doc(hidden)]
  project_info: ProjectInfo,
}

impl TemplateMeta {
  fn new(
    project_info: &ProjectInfo,
    template_options: &TemplateOptions,
  ) -> Self {
    println!("\nProjectInfo: {:?}", project_info);
    println!("TemplateOptions: {:?}\n", template_options);

    if let TemplateOptions::Remote(opts) = template_options {
      // Download template if it's a remote template.
      TemplateMeta::load_remote(opts).unwrap();
    }

    TemplateMeta {
      config: TemplateConfig::new(template_options.path(), &project_info.name),
      template_options: template_options.clone(),
      project_info: project_info.clone(),
    }
  }

  /// Clone remote repo into local path.
  fn load_remote(git_opts: &GitOptions) -> Result<()> {
    println!("Cloning remote repo into {}", git_opts.path());

    match git_opts.clone_repo() {
      Ok(_) => {}
      Err(err) => panic!("Could not create template: {}", err),
    }
    Ok(())
  }
}

impl TemplateMeta {}

impl Default for TemplateMeta {
  fn default() -> TemplateMeta {
    TemplateMeta {
      template_options: TemplateOptions::default(),
      config: TemplateConfig::default(),
      project_info: ProjectInfo::default(),
    }
  }
}

impl Drop for TemplateMeta {
  fn drop(&mut self) {
    // Delete cloned template, if `template_option` is `TemplateOptions::Remote`.
    match &self.template_options {
      TemplateOptions::Remote(git_opts) => {
        // Delete cloned repo.
        println!("Cleaning up clone templates...");
        git::delete_local_repo(&git_opts.path()).unwrap();
      }
      TemplateOptions::Local(_) => {}
    }
  }
}
