use crate::error::{Error, ErrorKind, Result};

use cargo::{
  core::GitReference, sources::git::GitRemote, util::config::Config,
  CargoResult,
};
use git2::{Repository as GitRepository, RepositoryInitOptions};
use url::{ParseError, Url};

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
  pub fn new(url: Url, branch: Option<String>) -> GitOptions {
    GitOptions {
      remote: url,
      branch: branch
        .map(GitReference::Branch)
        .unwrap_or(GitReference::DefaultBranch),
    }
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
