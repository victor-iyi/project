mod authors;
pub mod cli;
pub mod emoji;
mod error;
pub mod git;
mod info;
pub mod template;
mod util;

// Exported public API.
pub use self::cli::{Arguments, Cli};
pub use self::error::{Error, ErrorKind, Result};
pub use self::template::Template;
