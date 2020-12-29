use std::path::PathBuf;
use std::str::FromStr;

use serde::Serialize;

pub mod git;
pub mod guidon;
mod helpers;

/// Project template fields.
#[derive(Serialize)]
pub struct TemplateVariables {
  /// Name of the project, project directory, python module, model name, etc.
  name: String,
  /// Path where project is created. It defaults to the current directory
  /// and uses the Template's name.
  path: PathBuf,
  /// Project template engine to use. Defaults to `tf` [`tf`, `keras`].
  engine: TemplateEngine,
  /// Google Cloud Runtime version.
  runtime: f32,
  /// Python version.
  py_version: f32,
  /// GCS bucket name.
  bucket: String,
}

/// Available template engines.
#[derive(Serialize)]
enum TemplateEngine {
  Tf,
  Keras,
}

impl FromStr for TemplateEngine {
  type Err = &'static str;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "tf" | "TF" => Ok(TemplateEngine::Tf),
      "keras" | "Keras" | "KERAS" => Ok(TemplateEngine::Keras),
      _ => Err("no match"),
    }
  }
}
