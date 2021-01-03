use lotlinx::template::guidon::Guidon;
use lotlinx::{Error, ErrorKind, Result, Template};

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ToString;

use serde::Serialize;

#[derive(Serialize)]
enum Engine {
  Tf,
  Keras,
}

impl FromStr for Engine {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s {
      "tf" | "TF" => Ok(Engine::Tf),
      "keras" => Ok(Engine::Keras),
      _ => Err(Error::new(
        ErrorKind::Error,
        &format!("Invalid Engine {}", s),
      )),
    }
  }
}

impl ToString for Engine {
  fn to_string(&self) -> String {
    match self {
      Engine::Tf => "tf".to_string(),
      Engine::Keras => "keras".to_string(),
    }
  }
}

/// Project template fields.
#[derive(Serialize)]
pub struct PricingData {
  /// Name of the project, project directory, python module, model name, etc.
  name: String,
  /// Project description.
  description: String,
  /// Project template engine to use. Defaults to `tf` [`tf`, `keras`].
  engine: Engine,
  /// Google Cloud Runtime version.
  runtime: String,
  /// Python version.
  py_version: String,
  /// GCS bucket name.
  bucket: String,
}

impl Default for PricingData {
  fn default() -> PricingData {
    PricingData {
      name: "pricing".to_string(),
      description: "Pricing model".to_string(),
      engine: Engine::Tf,
      bucket: "lotlinxdata".to_string(),
      runtime: "2.1".to_string(),
      py_version: "3.7".to_string(),
    }
  }
}

fn main() {
  // let template: Template<&str, PricingData> =
  //   Template::new("/Users/victor/dev/template", "/Users/victor/dev/pricing");

  // With Template.
  let mut template = Template::with_data(
    "/Users/victor/dev/template",
    "/Users/victor/dev/pricing",
    PricingData::default(),
  );
  template.ignore_paths(&["/Users/victor/dev/template/venv"]);

  template
    .generate()
    .unwrap_or_else(|e| panic!("Problem generating template: {}", e));

  // With Guidon.
  let mut guidon = Guidon::new(PathBuf::from("~/dev/template"));

  let mut vars = BTreeMap::new();
  vars.insert("name".to_string(), "pricing".to_string());
  vars.insert("description".to_string(), "Pricing model".to_string());
  vars.insert("engine".to_string(), "tf".to_string());
  vars.insert("bucket".to_string(), "lotlinxdata".to_string());
  vars.insert("runtime".to_string(), "2.1".to_string());
  vars.insert("py_version".to_string(), "3.7".to_string());

  guidon.variables(vars);
  guidon.apply_template("~/dev/pricing").unwrap();
}
