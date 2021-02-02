#![allow(dead_code)]

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::{
  error::Result,
  template::{engine::Engine, parser},
  Error, ErrorKind,
};

/// Default template file containing variable template substitution.
pub(crate) const TEMPLATE_FILE: &str = "template.toml";

#[derive(Deserialize)]
pub(crate) struct TemplateConfig {
  /// Replace these variable keys with their value in template files.
  pub(crate) variables: Option<HashMap<String, String>>,
  /// The files you want to include as template.
  pub(crate) filters: Filters,
  /// Files or folders to rename.
  pub(crate) rename: Option<HashMap<String, String>>,
  /// Templating engine information.
  pub(crate) engine: Option<Engine>,
}

impl TemplateConfig {
  /// Create & parse the `"template.toml"` file in the project base directory.
  pub fn new(template_dir: &Path, project_name: &str) -> TemplateConfig {
    match Self::parse(&template_dir, project_name) {
      Ok(config) => config,
      Err(err) if err.kind() == &ErrorKind::NotFound => {
        eprintln!("Using default template configurations: {}", err);
        TemplateConfig::default()
      }
      Err(err) => {
        panic!("ERROR: {}", err);
      }
    }
  }

  /// Parse a given `template.toml` file as substitute all default variables.
  ///
  /// Return as a `Result<TemplateConfig>` for successful and parse failure.
  fn parse(template_dir: &dyn AsRef<Path>, project_name: &str) -> Result<Self> {
    let template_path = template_dir.as_ref().join(TEMPLATE_FILE);
    if !template_path.exists() {
      return Err(Error::new(ErrorKind::NotFound, "No template file."));
    }
    // Parsed template string.
    let parsed = parser::parse_template_file(&template_path, project_name)?;

    // Deserialize the `template.toml` file into `TemplateConfig`.
    let mut config: TemplateConfig = toml::from_str(&parsed)?;

    // Assert both `include` & `exclude` isn't both provided.
    if config.filters.include.is_some() && config.filters.exclude.is_some() {
      config.filters.exclude = None;
      eprintln!(
        "One of `include` or `exclude` should be provided, but not both."
      )
    }

    // Return the parsed configuration.
    Ok(config)
  }
}

impl Default for TemplateConfig {
  fn default() -> TemplateConfig {
    TemplateConfig {
      variables: None,
      rename: None,
      filters: Filters::default(),
      engine: Some(Engine::default()),
    }
  }
}

/// Files or Directories to be included or ignored while parsing
/// templates.
#[derive(Debug, Deserialize)]
pub(crate) struct Filters {
  /// The files you want to include in generated projects.
  pub(crate) include: Option<Vec<String>>,
  /// Directories & files to exlucde (e.g: .git, .idea, .DS_Store, etc.)
  pub(crate) exclude: Option<Vec<String>>,
}

impl Default for Filters {
  fn default() -> Filters {
    Filters {
      include: None,
      // Exclude these dirs by default.
      exclude: Some(vec![
        "venv".to_string(),
        ".git".to_string(),
        ".idea".to_string(),
        ".vscode".to_string(),
      ]),
    }
  }
}
