use crate::error::Result;

use cargo::{
  core::GitReference, sources::git::GitRemote, util::config::Config,
  CargoResult,
};
use git2::{Repository as GitRepository, RepositoryInitOptions};
use tempfile::Builder;
use url::Url;

use std::{fs, path::Path};

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

  pub fn path(&self) -> &str {
    self.remote.path()
  }

  /// Fetch template from a remote path into a `template_dir`.
  pub fn create(&self, template_dir: &Path) -> CargoResult<String> {
    crate::git::create(template_dir, self)
  }

  pub fn branch(&self) -> String {
    match &self.branch {
      GitReference::Branch(b) => b.to_owned(),
      GitReference::DefaultBranch => {
        self.get_default_branch().expect("Unable to fetch `HEAD`.")
      }
      _ => {
        unreachable!()
      }
    }
  }

  fn get_default_branch(&self) -> Result<String> {
    let repo = GitRepository::init(self.path())?;
    let mut git_remote = repo.remote_anonymous(self.remote.as_str())?;
    git_remote.connect(git2::Direction::Fetch)?;
    let default_branch = git_remote.default_branch()?;
    let branch_name = default_branch
      .as_str()
      .unwrap_or("refs/heads/master")
      .replace("refs/heads/", "");
    Ok(branch_name)
  }

  /// Initializes a new repository from a given git `branch` into a `project_dir`.
  pub fn init(
    &self,
    project_dir: &Path,
    branch: &str,
  ) -> Result<GitRepository> {
    let mut opt = RepositoryInitOptions::new();
    opt.bare(false);
    opt.initial_head(branch);

    Ok(
      GitRepository::init_opts(project_dir, &opt)
        .unwrap_or_else(|_| panic!("Couldn't init new repository")),
    )
  }
}

/// Fetch a `GitRemote` into a `template_dir`.
pub fn create(template_dir: &Path, opts: &GitOptions) -> CargoResult<String> {
  let temp = Builder::new().prefix(template_dir).tempdir()?;
  let config = Config::default()?;
  let remote = GitRemote::new(&opts.remote);

  // Checkout repo branch.
  let ((db, rev), branch_name) = match &opts.branch {
    GitReference::Branch(branch_name) => (
      remote.checkout(&temp.path(), None, &opts.branch, None, &config)?,
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
      let repo = GitRepository::init(&temp.path())?;
      let mut git_remote = repo.remote_anonymous(remote.url().as_str())?;
      git_remote.connect(git2::Direction::Fetch)?;
      let default_branch = git_remote.default_branch()?;
      let branch_name = default_branch
        .as_str()
        .unwrap_or("refs/heads/master")
        .replace("refs/heads/", "");
      (
        remote.checkout(
          &temp.path(),
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
  db.copy_to(rev, template_dir, &config)?;

  // Remove the ".git" files.
  fs::remove_dir_all(template_dir.join(".git"))
    .unwrap_or_else(|_| panic!("Error cleaning up cloned template"));

  Ok(branch_name)
}

/// Clean up all items in `.git/` folder in a cloned git repo.
pub fn remove_history(template_dir: &Path) -> Result<()> {
  fs::remove_dir_all(template_dir.join(".git"))
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

/// Delete temporary template repo from base `template_dir`.
pub fn delete_local_repo(template_dir: &Path) -> Result<()> {
  fs::remove_dir_all(template_dir)
    .unwrap_or_else(|_| panic!("Error cleaning up git repo"));

  Ok(())
}
