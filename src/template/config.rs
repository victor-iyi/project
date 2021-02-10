#![allow(dead_code)]

use std::{collections::HashMap, path::Path};

use console::style;
use serde::Deserialize;

use crate::{emoji, error::Result, template::parser, Error, ErrorKind};

/// Default template file containing variable template substitution.
pub(crate) const TEMPLATE_FILE: &str = "template.toml";

#[derive(Debug, Deserialize)]
pub(crate) struct TemplateConfig {
  /// Replace these variable keys with their value in template files.
  pub(crate) variables: Option<HashMap<String, String>>,
  /// The files you want to include as template.
  pub(crate) filters: Option<Filters>,
  /// Files or folders to rename.
  pub(crate) rename: Option<HashMap<String, String>>,
}

impl TemplateConfig {
  /// Create & parse the `"template.toml"` file in the project base directory.
  pub(crate) fn new(template_dir: &Path, project_name: &str) -> TemplateConfig {
    match Self::parse(&template_dir, project_name) {
      Ok(config) => config,
      Err(err) if err.kind() == &ErrorKind::NotFound => {
        eprintln!(
          "{} {}",
          emoji::SHRUG,
          style("Using default template configurations")
            .bold()
            .yellow()
        );
        TemplateConfig::default()
      }
      Err(err) => {
        panic!(
          "{} {} {}",
          emoji::ERROR,
          style("ERROR:").bold().red(),
          style(err).bold().red()
        );
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
    match &mut config.filters {
      Some(f) if f.include.is_some() && f.exclude.is_some() => {
        f.exclude = None;
        eprintln!(
          "{} {}",
          emoji::WARN,
          style(
            "One of `include` or `exclude` should be provided, but not both."
          )
          .bold()
          .yellow()
        );
      }
      Some(_) => (),
      None => (),
    };

    // Return the parsed configuration.
    Ok(config)
  }
}

impl Default for TemplateConfig {
  fn default() -> TemplateConfig {
    TemplateConfig {
      variables: None,
      rename: None,
      filters: Some(Filters::default()),
    }
  }
}

/// Files or Directories to be included or ignored while parsing
/// templates.
#[derive(Debug, Clone, Deserialize)]
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
