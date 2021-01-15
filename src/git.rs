use crate::error::{Error, ErrorKind, Result};

use cargo::{
  core::GitReference, sources::git::GitRemote, util::config::Config,
  CargoResult,
};
use git2::{Repository as GitRepository, RepositoryInitOptions};
use url::Url;

use std::{
  env, fs,
  path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct GitOptions {
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

/// Checksout a `GitRemote` into a temporary folder.
pub(crate) fn create(
  project_dir: &Path,
  opts: GitOptions,
) -> CargoResult<String> {
  let dest = env::temp_dir().join(project_dir);
  let config = Config::default()?;
  let remote = GitRemote::new(&opts.remote);

  // Checkout repo branch.
  let ((db, rev), branch_name) = match &opts.branch {
    GitReference::Branch(branch_name) => (
      remote.checkout(&dest, None, &opts.branch, None, &config)?,
      branch_name.clone(),
    ),
    GitReference::DefaultBranch => {
      // Cargo has a specific behavior for now for handling the "default" branch. It forces
      // it to the branch named "master" even if the actual default branch of the repository
      // is something else. They intent to change this behavior in the future but they don't
      // want to break the compactibility.
      //
      // See issues:
      //  - https://github.com/rust-lang/cargo/issues/8364
      //  - https://github.com/rust-lang/cargo/issues/8468
      let repo = GitRepository::init(&dest)?;
      let mut git_remote = repo.remote_anonymous(remote.url().as_str())?;
      git_remote.connect(git2::Direction::Fetch)?;
      let default_branch = git_remote.default_branch()?;
      let branch_name = default_branch
        .as_str()
        .unwrap_or("refs/heads/master")
        .replace("refs/heads/", "");
      (
        remote.checkout(
          &dest,
          None,
          &GitReference::Branch(branch_name.clone()),
          None,
          &config,
        )?,
        branch_name,
      )
    }
    _ => unreachable!(),
  };

  // This clones the remote and handles all the submodules.
  db.copy_to(rev, project_dir, &config)?;
  Ok(branch_name)
}

/// Clean up all items in `.git/` folder in a cloned git repo.
pub fn remove_history(project_dir: &Path) -> Result<()> {
  fs::remove_dir_all(project_dir.join(".git"))
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

/// Delete repo.
pub(crate) fn delete_local_repo(project_dir: &Path) -> Result<()> {
  fs::remove_dir_all(project_dir)
    .unwrap_or_else(|_| panic!("Error cleaning up git repo"));

  Ok(())
}
