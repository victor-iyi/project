use console::style;
use walkdir::{DirEntry, WalkDir};

use crate::{
  cli::{Arguments, Cli},
  emoji,
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
  fmt, fs,
  ops::Deref,
  path::{Path, PathBuf},
};

pub(crate) mod config;
pub(crate) mod engine;
pub(crate) mod helpers;
pub(crate) mod parser;

/// Template builds and generates the project from a given template.
///
/// # Example
///
/// ```rust
/// use project::{ProjectInfo, TemplateOptions, Template};
///
/// # #[allow(clippy::needless_doctest_main)]
/// fn main() {
///   let project = ProjectInfo::from("my-project");
///   let options = TemplateOptions::new("victor-iyi/project", None);
///
///   let template = Template::new(&project, &options);
/// # std::fs::remove_dir_all(&project.path()).unwrap();
/// }
/// ```
pub struct Template {
  #[doc(hidden)]
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
  /// Most important function for this entire library is this method.
  ///
  /// This is where all the parts come together to build new project from a template source,
  /// walking through the source directories and performing one of three tasks:
  /// - Creating a new directory in target location if entry is a directory.
  /// - Copying files over to the target location if entry is a regular file.
  /// - Performing template rendering if entry is a template file.
  ///
  /// It also applies necessary configurations: like filtering excluded files & directories,
  /// renaming target files and directories and many more.
  ///
  /// # Example
  ///
  /// ```rust, no_run
  /// use project::{ProjectInfo, TemplateOptions, Template};
  ///
  /// # #[allow(clippy::needless_doctest_main)]
  /// # fn main() {
  ///   let project = ProjectInfo::from("my-project");
  ///   let options = TemplateOptions::new("victor-iyi/project", None);
  ///
  ///   let template = Template::new(&project, &options);
  ///   assert!(&template.generate().is_ok());
  /// # std::fs::remove_dir_all(&project.path()).unwrap();
  /// # }
  /// ```
  pub fn generate(&self) -> Result<()> {
    // Project path.
    let project_dir = &self.project_info.path;
    // Template path.
    let template_dir = &self.template_options.path();

    // Walk the `template_dir`.
    for entry in WalkDir::new(template_dir)
      .into_iter()
      .filter_entry(|e| !self.filter_ignore(e))
      .filter_map(|e| e.ok())
    {
      // Strip `template_dir` from entry.
      let relative_path = entry.path().strip_prefix(template_dir)?;
      // Append stripped path to `project_dir`.
      let target = self.rename_path(relative_path, project_dir);

      if entry.path().is_dir() {
        fs::create_dir_all(&target)?;
      } else {
        self.substitute(entry.path(), &target)?;
      }
    }

    println!("{} {}", emoji::SPARKLE, style("Finished!").bold().green(),);
    println!(
      "{} \"{}\"",
      style("Project created in: ").bold().white(),
      style(&self.project_info.path().display()).bold().yellow()
    );

    Ok(())
  }

  /// Rename path based on the config file i.e. `"template.toml"` file.
  /// If there's no `[rename]` clause in the template file, the template
  /// filename is used instead.
  ///
  /// # Example
  ///
  /// ```toml
  /// # template.toml
  /// [rename]
  /// template = {{ project-name }}
  /// bin = "scripts"
  /// ```
  ///
  /// `{{ project-name }}` will be resolved to whatever the project's name is
  /// e.g `"my_project"`. Therefore, `path/to/template/file` will be renamed
  /// to `path/to/my_project/file`. Same with `bin` which will be renamed
  /// to `scripts`.
  fn rename_path(&self, relative_path: &Path, project_dir: &Path) -> PathBuf {
    let maps = self.rename_maps();
    if maps.is_empty() {
      // Append stripped path to `project_dir`.
      project_dir.join(relative_path)
    } else {
      // Go through the `maps` & rename paths accordingly.
      let mut rel_path = relative_path.to_path_buf();
      for (key, value) in &maps {
        // If `key` occurs in `rel_path`, replace the occurrenc with `value`.
        let renamed: PathBuf = rel_path
          .iter()
          .map(|path| -> &str {
            if key == path.to_str().unwrap() {
              value
            } else {
              path.to_str().unwrap()
            }
          })
          .collect();

        if renamed != rel_path {
          rel_path = renamed;
        }
      }
      // Append `rel_path` to `project_dir`.
      project_dir.join(rel_path)
    }
  }

  /// Template substitution is done here, based on the `src` file.
  ///
  /// If the `src` file or the template file has extensions supported by [`Engine`],
  /// template substitution will be done for such file, otherwise the file is just
  /// copied over to the `dest` or target/project file.
  ///
  /// **NOTE:** For files without extension; if you want it to be templated, append
  /// any extension supported by [`Engine`] e.g `".hbs"` or `".liquid"` as it's extension.
  /// It will be parsed and the extension will be dropped before writing to the target
  /// file.
  ///
  /// See [`Engine`] for more details.
  ///
  /// [`Engine`]: struct.Engine
  fn substitute(&self, src: &Path, dest: &Path) -> Result<()> {
    if let Some(ext) = src.extension() {
      let engine = Engine::new(ext);

      engine.render(src, &dest, &self.variables())?;
    } else {
      // Copy over file without extension. If you want it to be
      // templated, append ".hbs" or ".liquid" as extension.
      fs::copy(src, dest)?;
    }

    Ok(())
  }

  fn filter_ignore(&self, entry: &DirEntry) -> bool {
    // Filterignored/included files here...
    let (should_ignore, files) = self.get_ignored();

    if should_ignore {
      entry
        .file_name()
        .to_str()
        .map(|s| files.contains(&s.to_string()))
        .unwrap_or(false)
    } else {
      !entry
        .file_name()
        .to_str()
        .map(|s| files.contains(&s.to_string()))
        .unwrap_or(false)
    }
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

impl fmt::Debug for Template {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Template")
      .field("option", &self.template_options)
      .field("project", &self.project_info)
      .field("config", &self.config)
      .finish()
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
    if let TemplateOptions::Remote(opts) = template_options {
      // Download template if it's a remote template.
      TemplateMeta::load_remote(opts);
    }

    TemplateMeta {
      config: TemplateConfig::new(
        &template_options.path(),
        &project_info.name_snake_case(),
      ),
      template_options: template_options.clone(),
      project_info: project_info.clone(),
    }
  }

  /// Clone remote repo into local path.
  fn load_remote(git_opts: &GitOptions) {
    println!(
      "{} {} {}",
      emoji::WRENCH,
      style("Cloning remote repo into ").bold(),
      style(&git_opts.path().display()).bold().white()
    );

    match git_opts.clone_repo() {
      Ok(_) => {}
      Err(err) => panic!(
        "{} {} {}",
        emoji::ERROR,
        style("Could not create template:").bold().red(),
        style(err).bold().red()
      ),
    }
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

  pub(crate) fn get_ignored(&self) -> (bool, Vec<String>) {
    let filters = match &self.config.filters {
      Some(f) => f,
      None => panic!("No Filters."),
    };
    if filters.include.is_some() {
      (true, filters.include.clone().unwrap())
    } else {
      (true, filters.exclude.clone().unwrap())
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
        println!(
          "{} {}",
          emoji::WRENCH,
          style("Cleaning up cloned templates...").bold().yellow()
        );
        git::delete_local_repo(&git_opts.path()).unwrap();
      }
      TemplateOptions::Local(_) => {}
    }
  }
}
