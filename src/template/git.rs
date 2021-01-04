use crate::error::{Error, ErrorKind, Result};
use crate::template::guidon::{Guidon, TryNew};

use derive_builder::Builder;
use git2::build::RepoBuilder;
use git2::{AutotagOption, Cred, CredentialType, FetchOptions, ProxyOptions};
use log::debug;
use url::Url;

use std::env;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;

/// GitOptions for git initialization.
/// cf [GitOptionsBuilder](struct.GitOptionsBuilder.html) documentation.
///
/// A repo URL must be given
/// Optional properties:
/// * `rev`: a revision. master by default, can be a tag.
/// * `unsecure`: set to true to skip certificate check
/// * `auto_proxy`: set to true to let git discovers proxy configuration
#[derive(Debug, Clone, Builder, Default)]
#[builder(default, field(private), setter(into, strip_option))]
pub struct GitOptions {
  /// Repo url, with user / password if needed
  repo: String,
  /// The revision to retrieve (branch, tag...). master by default.
  /// A branch should be given as `origin/branch_name`
  /// A tag as `tag_name`, and a commit by id
  rev: Option<String>,
  /// if set to `true`, certificate validation will not be done. `false` by default
  unsecure: bool,
  /// if set to `true` will try to autodetect proxy configuration. `false` by default
  auto_proxy: bool,
}

impl GitOptions {
  /// Get a builder for GitOptions
  pub fn builder() -> GitOptionsBuilder {
    GitOptionsBuilder::default()
  }
}

impl<'a> TryNew<GitOptions> for Guidon<'a> {
  /// Initialization from a git repository
  /// The repo *MUST* contains at its root a `template.toml` file.
  fn try_new(git: GitOptions) -> Result<Self> {
    let mut cb = git2::RemoteCallbacks::new();

    cb.credentials(move |_url, _user_from_url, _cred| {
      if _cred.contains(CredentialType::USERNAME) {
        debug!("CredentialType::USERNAME");
        return Cred::username(_user_from_url.unwrap_or("git"));
      }

      if _cred.contains(CredentialType::USER_PASS_PLAINTEXT) {
        debug!("CredentialType::USER_PASS_PLAINTEXT");
        let url = Url::parse(_url).map_err(|e| {
          git2::Error::from_str(&format!("Malformed URL: {}", e.to_string()))
        })?;
        let user = url.username();
        let password = url.password().unwrap_or("");
        if user.is_empty() && password.is_empty() {
          return Cred::default();
        }
        return Cred::userpass_plaintext(user, password);
      }

      if _cred.contains(CredentialType::SSH_KEY) {
        // TODO: Test from in memory GPM_SSH_KEY ?
        debug!("CredentialType::SSH_KEY");
        return Cred::ssh_key_from_agent("git");
      }

      debug!("CredentialType::None");
      Err(git2::Error::from_str("No credential type found"))
    });

    if git.unsecure {
      // The certificate check must return true if the certificate is accepted.
      cb.certificate_check(|_, _| true);
    }

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb)
      .download_tags(AutotagOption::All)
      .update_fetchhead(true);

    if git.auto_proxy {
      let mut po = ProxyOptions::new();
      po.auto();
      fo.proxy_options(po);
    }

    let dest = env::temp_dir().join("guidon");
    let _ = remove_dir_all(&dest);
    create_dir_all(&dest)?;
    let repo = RepoBuilder::new()
      .fetch_options(fo)
      .clone(&git.repo, dest.as_path())?;
    let local = repo.revparse_single(
      &git.rev.unwrap_or_else(|| "refs/heads/master".to_owned()),
    )?;
    let mut opts = git2::build::CheckoutBuilder::new();
    opts.force();
    opts.use_theirs(true);
    repo.checkout_tree(&local, Some(&mut opts))?;

    Guidon::try_new(dest)
  }
}
