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
  fs::File,
  io::{BufReader, Read, Write},
  path::Path,
};

mod handlebars;
mod liquid;
mod regex;

/// Available templating engine.
#[derive(Deserialize)]
pub(crate) enum Engine {
  /// Use regular expression.
  RegEx(String),
  /// Handlebars with file extension: "hbs".
  Handlebars(String),
  /// Liquid templating engine with file extension: "liquid".
  Liquid(String),
}

impl Default for Engine {
  fn default() -> Engine {
    Engine::Handlebars(String::from("hbs"))
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
      Engine::RegEx(_) => regex::parse(&content, variables)?,
      Engine::Handlebars(_) => handlebars::parse(&content, variables)?,
      Engine::Liquid(_) => liquid::parse(&content, variables)?,
    };

    // Write new content into target file.
    let mut target_file = File::create(target)?;
    target_file.write_all(new_content.as_bytes())?;
    Ok(())
  }
}
