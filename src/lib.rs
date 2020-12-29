pub use self::error::{Error, ErrorKind, Result};
pub use render::Render;

pub mod cli;
mod error;
mod render;
pub mod template;
