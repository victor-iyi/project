//! Templating engines.
//!
//! - [Handlebars][handlebars]
//! - [Liquid][liquid]
//!
//! [handlebars]: https://handlebarsjs.com
//! [liquid]: https://shopify.github.io/liquid/
//!

use serde::Deserialize;

#[cfg(feature = "hbs")]
mod handlebars;

#[cfg(feature = "liquid")]
mod liquid;

/// Available templating engine.
#[derive(Deserialize)]
pub enum Engine {
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

trait TemplateEngine {

}
