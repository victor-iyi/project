//! Templating engines.
//!
//! - [Handlebars][handlebars]
//! - [Liquid][liquid]
//!
//! [handlebars]: https://handlebarsjs.com
//! [liquid]: https://shopify.github.io/liquid/
//!

use std::path::Path;

#[cfg(feature = "hbs")]
mod handlebars;

#[cfg(feature = "liquid")]
mod liquid;

/// Templating engine trait.
pub trait Engine {
  fn replace_all(project_dir: dyn AsRef<Path>);
}
