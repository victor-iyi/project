use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::util;
use crate::{engine::Engine, substitution};

/// Default template file containing variable template substitution.
pub const TEMPLATE_FILE: &str = "template.toml";

#[derive(Deserialize)]
pub struct TemplateConfig {
  /// Replace these variable keys with their value in template files.
  pub variables: Option<HashMap<String, String>>,
  /// The files you want to include as template.
  pub include: Option<Vec<String>>,
  /// Directories & files to exclude (.git, .idea, .DS_Store, etc.)
  pub exclude: Option<Vec<String>>,
  /// Templating engine information.
  pub engine: Engine,
}

impl TemplateConfig {
  /// Create a new `TemplateConfig` from `config::TEMPLATE_FILE`.
  pub fn new(path: &dyn AsRef<Path>) -> Self {
    match Self::parse(&path.as_ref().join(TEMPLATE_FILE)) {
      Ok(config) => config,
      Err(_) => TemplateConfig::default(),
    }
  }

  /// Parse a given `template.toml` file as substitute all default variables.
  ///
  /// Return as a `Result<TemplateConfig>` for successful and parse failure.
  pub(crate) fn parse(path: &dyn AsRef<Path>) -> Result<Self> {
    // Parsed template string.
    let parsed = substitution::parse_template_file(path, util::filename(path))?;

    // Deserialize the `template.toml` file into `TemplateConfig`.
    let mut config: TemplateConfig = toml::from_str(&parsed)?;

    // Assert both `include` & `exclude` isn't both provided.
    if config.include.is_some() && config.exclude.is_some() {
      config.exclude = None;
      eprintln!(
        "One of `include` or `exclude` should be provided, but not both."
      )
    }

    // Return the parsed configuration.
    Ok(config)
  }
}

impl FromStr for TemplateConfig {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    Self::parse(&Path::new(s))
  }
}

impl From<&dyn AsRef<Path>> for TemplateConfig {
  fn from(path: &dyn AsRef<Path>) -> Self {
    match Self::parse(path) {
      Ok(config) => config,
      Err(_) => TemplateConfig::default(),
    }
  }
}

impl Default for TemplateConfig {
  fn default() -> TemplateConfig {
    TemplateConfig {
      variables: None,
      include: None,
      exclude: None,
      engine: Engine::default(),
    }
  }
}
