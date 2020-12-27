mod cli;
mod error;
mod template;

/// Lotlinx Result type.
pub type Result<T> = std::result::Result<T, error::Error>;
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
