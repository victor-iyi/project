use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use handlebars::Handlebars;
use serde::Serialize;
use walkdir::WalkDir;

use crate::{Error, ErrorKind, Result};

#[cfg(feature = "git")]
pub mod git;
pub mod guidon;
mod helpers;

/// Project template fields.
#[derive(Serialize)]
pub struct TemplateData {
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

pub struct Template<P: AsRef<Path>, D: Serialize> {
  /// Source path where `hbs` template exist.
  src_path: P,

  /// Path where template will be created.
  dest_path: P,

  /// Data that will be written from `src_path` to `dest_path`.
  data: Option<D>,
}

impl<P: AsRef<Path>, D: Serialize> Template<P, D> {
  /// Create a new `Template<T>` with `data` set to `None`.
  pub fn new(src_path: P, dest_path: P) -> Template<P, D> {
    Template {
      src_path,
      dest_path,
      data: None,
    }
  }
}

impl<P: AsRef<Path>, D: Serialize> Template<P, D> {
  pub fn generate(&self) -> Result<()> {
    // Check if `src_path` is not a directory.
    if !self.src_path.as_ref().exists() {
      return Err(Error::new(
        ErrorKind::NotFound,
        &format!("{} was not found.", self.src_path.as_ref().display()),
      ));
    }

    // Source path was not a directory.
    if !self.src_path.as_ref().is_dir() {
      return Err(Error::new(
        ErrorKind::NotADirectory,
        &format!("{} is a directory.", self.src_path.as_ref().display()),
      ));
    }

    // Create destination directory.
    fs::create_dir_all(&self.dest_path.as_ref())?;

    let _hb = self.register_helpers(true);
    println!("Data empty? {}", self.data.is_none());

    // Walk the source directory
    // If it's a directory, create the target directory.
    // if it's a file:
    // check if it ends with hbs: call handlebars and write the processed file to target
    // If no substitution, do a direct copy of file to dest path
    let walker = WalkDir::new(self.src_path.as_ref()).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
      // let dest = self.
      let src_path = entry.path();

      let dest_path = self.dest_path.as_ref().join(src_path);
      if src_path.is_file() {
        if src_path.ends_with("hbs") {
          let _src_template = fs::File::open(src_path)?;
        } else {
          let mut _out_file = fs::File::create(&dest_path)?;
          // hb.render_template_source_to_write(&mut , data, writer)
        }
      } else {
        println!("Neither a file nor a directory.");
      }
    }
    Ok(())
  }

  pub fn register_helpers(&self, strict_mode: bool) -> Handlebars {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(strict_mode);

    // Register handlebars helpers.
    handlebars.register_helper("replace", Box::new(helpers::replace));
    handlebars.register_helper("append", Box::new(helpers::append));
    handlebars.register_helper("prepend", Box::new(helpers::prepend));
    handlebars.register_helper("up", Box::new(helpers::up));
    handlebars.register_helper("low", Box::new(helpers::low));

    handlebars
  }
}
