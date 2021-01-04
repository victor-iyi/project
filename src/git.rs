use crate::error::{Error, ErrorKind, Result};

use cargo::core::GitReference;
use git2::{Repository as GitRepository, RepositoryInitOptions};
use url::Url;

use std::path::{Path, PathBuf};

pub(crate) struct GitOptions {
  /// Remote or local git URI.
  remote: Url,
  /// Git branch to select. Defaults to the `HEAD` branch.
  branch: GitReference,
}

impl GitOptions {
  /// Parses the `git` as URL or local git URI and returns a `GitOptions`.
  pub fn new(git: &str, branch: Option<String>) -> Result<Self> {
    // Parse the `git` URI.
    let remote = match GitOptions::parse_git(git) {
      Ok(u) => u,
      Err(e) => panic!("Failed parsing git {}: {}", git, e),
    };

    Ok(GitOptions {
      remote,
      branch: branch
        .map(GitReference::Branch)
        .unwrap_or(GitReference::DefaultBranch),
    })
  }

  /// Creates a new `GitOptions`, first with `new` and then as a GitHub `owner/repo` remote, like [hub]
  ///
  /// [hub]: https://github.com/github/hub
  pub fn with_abbr(git: &str, branch: Option<String>) -> Result<Self> {
    Self::new(git, branch.clone()).or_else(|e| {
      Self::new(&format!("https://github.com/{}.git", git), branch)
        .map_err(|_| e)
    })
  }

  /// Parses `git` URI as either remote or local path.
  fn parse_git(git: &str) -> Result<Url> {
    let remote = match Url::parse(git) {
      Ok(u) => u,
      Err(url::ParseError::RelativeUrlWithoutBase) => {
        let given_path = Path::new(git);
        let mut git_path = PathBuf::new();
        if given_path.is_relative() {
          git_path.push(std::env::current_dir()?);
          git_path.push(given_path);
          if !git_path.exists() {
            return Err(Error::new(
              ErrorKind::NotFound,
              &format!("path {} doesn't exist.", &git_path.display()),
            ));
          }
        } else {
          git_path.push(git);
        }

        Url::from_file_path(&git_path).map_err(|_| -> Error {
          Error::new(ErrorKind::GitError, &format!("[as file path] {}", git))
        })?
      }
      Err(err) => {
        return Err(Error::from(err));
      }
    };

    Ok(remote)
  }
}

/// Clean up all cloned git repo.
pub fn remove_history(project_dir: &Path) -> Result<()> {
  std::fs::remove_dir_all(project_dir.join(".git"))
    .unwrap_or_else(|_| panic!("Error cleaning up cloned template"));

  Ok(())
}

/// Initializes a new repository from a given git `branch` into a `project_dir`.
pub fn init(project_dir: &Path, branch: &str) -> Result<GitRepository> {
  let mut opt = RepositoryInitOptions::new();
  opt.bare(false);
  opt.initial_head(branch);

  Ok(
    GitRepository::init_opts(project_dir, &opt)
      .unwrap_or_else(|_| panic!("Couldn't init new repository")),
  )
}
