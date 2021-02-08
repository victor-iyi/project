use crate::{emoji, error::Result, util};

use cargo::core::GitReference;
use console::style;
use git2::{
  Cred, RemoteCallbacks, Repository as GitRepository, RepositoryInitOptions,
};
use url::Url;

use std::{env, fs, path::Path};

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

  #[inline]
  pub fn path(&self) -> &str {
    // self.remote.path().trim_start_matches('/')
    util::basename(self.remote.path())
  }

  pub fn clone_repo(&self) -> Result<()> {
    // let temp = Builder::new().prefix(template_dir).tempdir()?;
    // printnl!("Temporary dir: {}", temp.path());

    // Local path where remote repo will be cloned.
    let clone_path = Path::new(self.path());

    // Clone the project.
    // let _repo = match GitRepository::clone(self.remote.as_str(), clone_path) {
    //   Ok(repo) => repo,
    //   Err(e) => panic!("Failed to clone: {}", e),
    // };

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
      Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
        None,
      )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Create clone directory if it doesn't exist.
    if !clone_path.exists() {
      fs::create_dir_all(clone_path)?;
      // } else {
      //   // Remove the contents of the directory.
      //   fs::remove_dir_all(clone_path)?;
      //   fs::create_dir_all(clone_path)?;
    }

    // Clone the project.
    builder.clone(self.remote.as_str(), clone_path)?;

    // Remove ".git" folder in cloned repo.
    self.remove_git_history(clone_path);

    // Successfully cloned.
    Ok(())
  }

  #[inline]
  fn remove_git_history(&self, dir: &Path) {
    fs::remove_dir_all(dir.join(".git")).unwrap_or_else(|err| {
      eprintln!(
        "{} {} {}",
        emoji::WARN,
        style("Could not clean up git history: {}").bold().yellow(),
        style(err).bold().yellow()
      )
    });
  }

  pub fn branch(&self) -> String {
    match &self.branch {
      GitReference::Branch(b) => b.to_owned(),
      GitReference::DefaultBranch => {
        self.get_default_branch().unwrap_or_else(|_| {
          panic!(
            "{} {}",
            emoji::ERROR,
            style("Unable to fetch `HEAD`.").bold().red()
          )
        })
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
}

/// Initializes a new repository from a given git `branch` into a `project_dir`.
pub fn init(project_dir: &Path, branch: &str) -> Result<GitRepository> {
  let mut opt = RepositoryInitOptions::new();
  opt.bare(false);
  opt.initial_head(branch);

  Ok(
    GitRepository::init_opts(project_dir, &opt).unwrap_or_else(|_| {
      panic!(
        "{} {}",
        emoji::ERROR,
        style("Couldn't init new repository").bold().red()
      )
    }),
  )
}

/// Delete temporary template repo from base `template_dir`.
#[inline]
pub fn delete_local_repo(template_dir: &dyn AsRef<Path>) -> Result<()> {
  fs::remove_dir_all(template_dir).unwrap_or_else(|_| {
    panic!(
      "{} {}",
      emoji::ERROR,
      style("Could not delete local repo").bold().red()
    )
  });

  Ok(())
}
