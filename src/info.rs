use crate::{
  emoji,
  error::{Error, Result},
  git::GitOptions,
  util,
};

use console::style;
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
  pub fn new(p: &Path) -> Self {
    let path = PathBuf::from(p);

    // Create project directory.
    if !path.exists() {
      fs::create_dir_all(&path).unwrap();
    }

    // Return absolute form of `path`.
    let path = match path.canonicalize() {
      Ok(p) => p,
      Err(e) => panic!(
        "{} {} \"{}\" {}",
        emoji::ERROR,
        style("Could not resolve path: ").bold().red(),
        style(&path.display()).bold(),
        style(e).bold().red()
      ),
    };

    let mut name: String = util::filename(&path).into();
    // TODO: add flag for converting project name to kebab case.
    if true {
      name = name.to_snake_case();
    }

    println!(
      "{} {} {}",
      emoji::WRENCH,
      style("Creating project: ").bold().white(),
      style(&name).bold().yellow()
    );

    ProjectInfo { name, path }
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

  /// Get relative path.
  pub fn rel_path(&self) -> PathBuf {
    let path = self.path();
    let curr_dir = env::current_dir().unwrap_or_else(|_| ".".into());

    match util::diff_paths(&path, &curr_dir) {
      Some(p) => p,
      None => path,
    }
  }

  pub fn path_kebab_case(&self) -> String {
    self.path.to_str().unwrap().to_kebab_case()
  }
}

impl From<&str> for ProjectInfo {
  fn from(s: &str) -> Self {
    Self::new(&Path::new(s))
  }
}

impl From<&dyn AsRef<Path>> for ProjectInfo {
  fn from(p: &dyn AsRef<Path>) -> Self {
    Self::new(p.as_ref())
  }
}

impl Default for ProjectInfo {
  fn default() -> Self {
    let curr_dir = env::current_dir().unwrap_or_else(|_e| ".".into());

    Self::new(&curr_dir)
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

#[allow(dead_code)]
#[derive(Debug)]
pub enum RemoteSource {
  GitHub,
  GitLab,
  BitBucket,
}

#[allow(dead_code)]
impl RemoteSource {
  pub fn to_str(&self) -> &str {
    match self {
      RemoteSource::GitHub => "github",
      RemoteSource::GitLab => "gitlab",
      RemoteSource::BitBucket => "bitbucket",
    }
  }

  pub fn from_str(s: &str) -> Self {
    match s {
      "github" => Self::GitHub,
      "gitlab" => Self::GitLab,
      "bitbucket" => Self::BitBucket,
      _ => panic!(
        "{} {} {}",
        emoji::ERROR,
        style("Unknown source:").bold().red(),
        style(s).bold()
      ),
    }
  }

  pub fn get_remote(&self, username: &str, repo: &str) -> String {
    match self {
      RemoteSource::GitHub => {
        format!("https://github.com/{}/{}.git", username, repo)
      }
      RemoteSource::GitLab => {
        format!("https://gitlab.com/{}/{}.git", username, repo)
      }
      RemoteSource::BitBucket => {
        format!("https://{0}@bitbucket.org/{0}/{1}", username, repo)
      }
    }
  }
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
      Err(err) => panic!(
        "{} {} {}",
        emoji::ERROR,
        style("ERROR: ").bold().red(),
        style(err).bold().red()
      ),
    }
  }

  pub fn set_source(&self, _source: &str) {
    // TODO: Find a way to add source as part of the template's remote options.
    // self.source = RemoteSource::from_str(source);
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
            // TODO: Add `--source` flag to cli.
            let source = RemoteSource::GitHub;
            let path = match source {
              RemoteSource::GitHub => {
                format!("https://github.com/{}.git", path)
              }
              RemoteSource::GitLab => {
                format!("https://gitlab.com/{}.git", path)
              }
              RemoteSource::BitBucket => {
                // FIXME: Re-format for bit-bucket.
                // https://username@bitbucket.org/username/repo.git
                format!("https://username@bitbucket.org/{}.git", path)
              }
            };
            Self::parse_path(&path, branch)?
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
