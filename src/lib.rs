mod authors;
#[cfg(feature = "cli")]
pub mod cli;
pub mod engine;
mod error;
#[cfg(feature = "git")]
pub mod git;
mod info;
mod render;
mod substitution;
pub mod template;
pub(crate) mod util;

// Exported public API.
#[cfg(feature = "cli")]
pub use self::cli::Cli;
pub use self::engine::Engine;
pub use self::error::{Error, ErrorKind, Result};
pub use self::template::Template;
