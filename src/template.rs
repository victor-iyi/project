use walkdir::{DirEntry, WalkDir};

use crate::{
  cli::{Arguments, Cli},
  error::Result,
  git::{self, GitOptions},
  info::{ProjectInfo, TemplateOptions},
  template::config::TemplateConfig,
};

use std::{fs, path::Path};

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

      if entry.path().is_dir() {
        fs::create_dir_all(&target)?;
        println!("Create directory: {}", target.display());
        continue;
      } else {
        // TODO: Check for files eligible for variable substitution.
        if entry.path().ends_with("hbs") {
          // TODO: Perform template substitution.
          println!("Perform template substitution.");
        } else {
          // Copy over files.
          fs::copy(entry.path(), &target)?;
          println!("Copy files: {}", target.display());
        }
      }
    }

    Ok(())
  }

  pub fn filter_ignore(&self, entry: &DirEntry) -> bool {
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

impl From<Arguments> for Template {
  fn from(args: Arguments) -> Template {
    Template {
      template: TemplateMeta::new(&args.project, &args.template),
    }
  }
}

impl From<Cli<'_>> for Template {
  fn from(cli: Cli) -> Template {
    Self::from(cli.args)
  }
}

impl Default for Template {
  fn default() -> Template {
    Template {
      template: TemplateMeta::default(),
    }
  }
}

// impl Template {
//   pub fn generate(&self, dest: &dyn AsRef<Path>) -> Result<()> {
//     // Target destination where template will be created.
//     let target: PathBuf = dest.as_ref().to_owned();

//     // Create destination folders.
//     std::fs::create_dir_all(dest.as_ref())?;

//     // Walk the path and copy src path over to dest path.
//     for entry in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
//       if entry.path().is_dir() {
//         std::fs::create_dir_all(entry.path())?;
//         continue;
//       } else if entry.path().is_file() {
//         // Open the file.
//         std::fs::copy(entry.path(), &target)?;
//       } else {
//         eprintln!("Do not know what's happening here...");
//       }
//       println!("{}", &entry.path().display());
//     }
//     Ok(())
//   }
// }

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
        git::delete_local_repo(&git_opts.path()).unwrap();
      }
      TemplateOptions::Local(_) => {}
    }
  }
}
