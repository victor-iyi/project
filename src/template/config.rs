use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use crate::engine::Engine;
use crate::error::{Error, ErrorKind, Result};

const TEMPLATE_FILE: &str = "template.toml";

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
  pub fn new() -> Self {
    match Self::from_str(TEMPLATE_FILE) {
      Ok(config) => config,
      Err(_) => TemplateConfig::default(),
    }
  }
}

impl FromStr for TemplateConfig {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match fs::read_to_string(s) {
      Ok(contents) => {
        let mut config: TemplateConfig = toml::from_str(&contents)?;
        if config.include.is_some() && config.exclude.is_some() {
          config.exclude = None;
          eprintln!(
            "One of `include` or `exclude` should be provided, but not both."
          )
        }
        Ok(config)
      }
      Err(err) => match err.kind() {
        std::io::ErrorKind::NotFound => {
          Err(Error::new(ErrorKind::NotFound, "No template found."))
        }
        _ => panic!("{}", err),
      },
    }
  }
}

impl From<&dyn AsRef<Path>> for TemplateConfig {
  fn from(path: &dyn AsRef<Path>) -> Self {
    match Self::from_str(path.as_ref().to_str().unwrap_or_else(|| {
      panic!("Could not convert {} to `str`", path.as_ref().display())
    })) {
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
