use crate::{
  error::{Error, Result},
  git::{self, GitOptions},
  template::config::TemplateConfig,
  util,
};

use heck::{KebabCase, SnakeCase};
use url::{ParseError, Url};

use std::{
  env, fs,
  path::{Path, PathBuf},
};

/// Information about the new project to be created.
#[derive(Debug, Clone)]
pub struct ProjectInfo {
  /// The project name, which is extracted from the project's base directory.
  pub name: String,
  /// Base directory of the target project.
  pub path: PathBuf,
}

impl ProjectInfo {
  /// Create a new project info: given project local path.
  pub fn new(s: &dyn AsRef<Path>) -> Self {
    Self::from(s)
  }
}

impl ProjectInfo {
  /// Get the raw project name.
  pub fn raw(&self) -> String {
    self.name.to_owned()
  }

  /// Get the project name in Kebab case.
  pub fn name_kebab_case(&self) -> String {
    self.name.to_kebab_case()
  }

  /// Get the project name in snake case.
  pub fn name_snake_case(&self) -> String {
    self.name.to_snake_case()
  }

  /// Get owned project path.
  pub fn path(&self) -> PathBuf {
    self.path.clone()
  }

  pub fn path_kebab_case(&self) -> String {
    self.path.to_str().unwrap().to_kebab_case()
  }
}

impl From<&str> for ProjectInfo {
  fn from(s: &str) -> Self {
    Self {
      name: util::basename(s).into(),
      path: PathBuf::from(s),
    }
  }
}

impl From<&dyn AsRef<Path>> for ProjectInfo {
  fn from(p: &dyn AsRef<Path>) -> Self {
    Self {
      name: util::filename(p).into(),
      path: PathBuf::from(p.as_ref()),
    }
  }
}

impl Default for ProjectInfo {
  fn default() -> Self {
    let curr_dir = env::current_dir().unwrap_or_else(|_e| ".".into());

    Self {
      name: util::filename(&curr_dir).into(),
      path: curr_dir,
    }
  }
}

/// `TemplateOptions` describes the kind of template we are using,
/// either a remote template or a local template.
#[derive(Debug, Clone)]
pub enum TemplateOptions {
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
  pub fn new(path: &str, branch: Option<&str>) -> TemplateOptions {
    // https://github.com/username/repo
    // username/repo
    // relative/path/to/template
    match Self::parse_path(path, branch.map(|s| s.to_string())) {
      Ok(opts) => opts,
      Err(err) => panic!("ERROR: {}", err),
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
          Err(_err) => {
            // Short Git URI.
            Self::parse_path(&format!("https://github.com/{}", path), branch)?
          }
        }
      }
      Err(err) => {
        return Err(Error::from(err));
      }
    };

    println!("{:?}", opts);
    Ok(opts)
  }
}

impl TemplateOptions {
  pub fn path(&self) -> &Path {
    match self {
      TemplateOptions::Local(p) => p,
      TemplateOptions::Remote(g) => &Path::new(g.path()),
    }
  }
}

impl From<&dyn AsRef<Path>> for TemplateOptions {
  fn from(path: &dyn AsRef<Path>) -> TemplateOptions {
    TemplateOptions::new(path.as_ref().to_str().unwrap(), None)
  }
}

impl Default for TemplateOptions {
  fn default() -> Self {
    let curr_dir = env::current_dir().unwrap_or_else(|_| ".".into());

    Self::Local(curr_dir)
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
  pub fn new(
    project_info: &ProjectInfo,
    template_options: &TemplateOptions,
  ) -> Self {
    println!("\nProjectInfo: {:?}", project_info);
    println!("TemplateOptions: {:?}\n", template_options);

    // Create new project's directory.
    fs::create_dir_all(&project_info.path).unwrap();

    if let TemplateOptions::Remote(opts) = template_options {
      // Download template if it's a remote template.
      TemplateMeta::load_remote(opts, &template_options.path()).unwrap();
    }

    TemplateMeta {
      config: TemplateConfig::new(template_options.path(), &project_info.name),
      template_options: template_options.clone(),
      project_info: project_info.clone(),
    }
  }

  fn load_remote(git_opts: &GitOptions, template_dir: &Path) -> Result<()> {
    match git_opts.create(template_dir) {
      Ok(_branch) => {}
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
