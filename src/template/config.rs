use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::{
  engine::Engine, error::Result, template::parser, Error,
  ErrorKind,
};

/// Default template file containing variable template substitution.
pub(crate) const TEMPLATE_FILE: &str = "template.toml";

#[derive(Deserialize)]
pub struct TemplateConfig {
  /// Replace these variable keys with their value in template files.
  pub variables: Option<HashMap<String, String>>,
  /// The files you want to include as template.
  pub include: Option<Vec<String>>,
  /// Directories & files to exclude (.git, .idea, .DS_Store, etc.)
  pub exclude: Option<Vec<String>>,
  /// Templating engine information.
  pub engine: Option<Engine>,
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

impl Default for TemplateConfig {
  fn default() -> TemplateConfig {
    TemplateConfig {
      variables: None,
      include: None,
      exclude: None,
      engine: Some(Engine::default()),
    }
  }
}
