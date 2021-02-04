//! Templating engines.
//!
//! - [Handlebars][handlebars]
//! - [Liquid][liquid]
//!
//! [handlebars]: https://handlebarsjs.com
//! [liquid]: https://shopify.github.io/liquid/
//!

use crate::error::Result;

use serde::{Deserialize, Serialize};

use std::{
  collections::HashMap,
  ffi::OsStr,
  fs::{self, File},
  io::{BufReader, Read, Write},
  path::Path,
};

mod handlebars;
mod liquid;

/// Available templating engine.
#[derive(Deserialize)]
pub(crate) enum Engine {
  /// Handlebars with file extension: "hbs".
  Handlebars,
  /// Liquid templating engine with file extension: "liquid".
  Liquid,
  /// Regular file; no need to be parsed.
  None,
}

impl Engine {
  pub(crate) fn new(ext: &OsStr) -> Engine {
    if ext == "hbs" {
      Engine::Handlebars
    } else if ext == "liquid" {
      Engine::Liquid
    } else {
      Engine::None
    }
  }
}

pub(crate) trait TemplateEngine {
  type Data: Serialize;

  fn render(
    &self,
    src: &Path,
    target: &Path,
    variables: &Self::Data,
  ) -> Result<()>;
}

impl TemplateEngine for Engine {
  type Data = HashMap<String, String>;
  fn render(
    &self,
    src: &Path,
    target: &Path,
    variables: &Self::Data,
  ) -> Result<()> {
    // Read contents of src file.
    let template_file = File::open(src)?;
    let mut buf_reader = BufReader::new(template_file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    let new_content = match self {
      Engine::Handlebars => handlebars::parse(&content, variables)?,
      Engine::Liquid => liquid::parse(&content, variables)?,
      Engine::None => {
        // Move file over to target.
        fs::copy(src, target)?;
        return Ok(());
      }
    };

    // Rename the file. Get rid of ".hbs" or ".liquid".
    let target = target.with_extension("");

    // Write new content into target file.
    let mut target_file = File::create(target)?;
    target_file.write_all(new_content.as_bytes())?;
    Ok(())
  }
}
