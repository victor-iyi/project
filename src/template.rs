use walkdir::{DirEntry, WalkDir};

use crate::{
  cli::{Arguments, Cli},
  error::Result,
  git::{self, GitOptions},
  info::{ProjectInfo, TemplateOptions},
  template::{
    config::TemplateConfig,
    engine::{Engine, TemplateEngine},
  },
};

use std::{
  collections::HashMap,
  ffi::OsStr,
  fs,
  ops::Deref,
  path::{Path, PathBuf},
};

pub(crate) mod config;
pub(crate) mod engine;
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
    let project_dir = self.project_info.path.as_path();
    // Template path.
    let template_dir = self.template_options.path();

    // Walk the `template_dir`.
    for entry in WalkDir::new(template_dir)
      .into_iter()
      .filter_entry(|e| !self.filter_ignore(e))
      .filter_map(|e| e.ok())
    {
      // Strip `template_dir` from entry.
      let relative_path = entry.path().strip_prefix(template_dir)?;
      // Append stripped path to `project_dir`.
      let mut target = project_dir.join(relative_path);

      println!("Realtive path: {}", relative_path.display());
      println!("Target: {}", target.display());
      println!("Entry: {}\n", entry.path().display());

      // std::process::exit(0);
      // TODO: Check configuration for path (`target`) to rename.
      target = self.rename(&entry.path(), &target);

      if entry.path().is_dir() {
        fs::create_dir_all(&target)?;
        continue;
      } else if let Some(ext) = entry.path().extension() {
        self.substitute(ext, entry.path(), &target)?;
      // println!("File: {}", entry.path().display());
      } else {
        println!("===Entry: {}===", entry.path().display());
        fs::copy(entry.path(), &target)?;
      }
    }

    println!("Done generating template into {}", project_dir.display());
    Ok(())
  }

  fn rename(&self, _entry: &Path, target: &Path) -> PathBuf {
    let maps = self.rename_maps();
    if maps.is_empty() {
      PathBuf::from(target)
    } else {
      // TODO: Go through the `maps` & rename paths accordingly.
      for (_key, _value) in &maps {
        // If `key` occurs in `target`
        // Replace `value` with `key` in `target`.
      }
      PathBuf::from(target)
    }
  }

  fn substitute(&self, ext: &OsStr, src: &Path, dest: &Path) -> Result<()> {
    let engine = Engine::new(ext);

    engine.render(src, &dest, &self.variables())?;

    Ok(())
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

impl Deref for Template {
  type Target = TemplateMeta;
  fn deref(&self) -> &Self::Target {
    &self.template
  }
}

/// Template & project comes together to load the template from remote or local
/// path, loads the `"template.toml"` config file, and initializes git for the
/// new project.
pub struct TemplateMeta {
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

impl TemplateMeta {
  pub(crate) fn variables(&self) -> HashMap<String, String> {
    match &self.config.variables {
      Some(var) => var.clone(),
      None => HashMap::new(),
    }
  }

  pub(crate) fn rename_maps(&self) -> HashMap<String, String> {
    match &self.config.rename {
      Some(rename) => rename.clone(),
      None => HashMap::new(),
    }
  }
}

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
