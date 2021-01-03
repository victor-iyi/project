use std::{fmt, io, str::FromStr};

/// Lotlinx Result type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
  /// Cannot find a file or directory.
  NotFound,

  /// Expected a folder/directory.
  NotADirectory,

  /// I/O error.
  Io,

  /// Error returned fro `std::path` if the prefix was not found.
  StripPrefix,

  /// Error related to git
  GitError,

  /// Generic error kind.
  Error,
}

/// Base error type.
pub struct Error {
  err: ErrorImpl,
}

impl Error {
  /// Create a new `Error` with error kind and failure message.
  pub fn new(kind: ErrorKind, message: &str) -> Error {
    Error {
      err: ErrorImpl::new(kind, message),
    }
  }

  // pub fn from_str(message: &str) -> Error {
  //   Error::new(ErrorKind::Error, message)
  // }
}

impl Error {
  pub fn kind(&self) -> &ErrorKind {
    &self.err.kind
  }

  pub fn message(&self) -> &str {
    &self.err.msg
  }
}

impl FromStr for Error {
  type Err = Error;

  #[doc(hidden)]
  fn from_str(message: &str) -> Result<Error> {
    Ok(Error::new(ErrorKind::Error, message))
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.err.msg)
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Error")
      .field("kind", &self.err.kind)
      .field("message", &self.err.msg)
      .finish()
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::new(ErrorKind::Io, &err.to_string())
  }
}

impl From<toml::de::Error> for Error {
  fn from(err: toml::de::Error) -> Self {
    Error::new(ErrorKind::StripPrefix, &err.to_string())
  }
}

impl From<handlebars::TemplateRenderError> for Error {
  fn from(err: handlebars::TemplateRenderError) -> Self {
    Error::new(ErrorKind::StripPrefix, &err.to_string())
  }
}

impl std::error::Error for Error {}

struct ErrorImpl {
  kind: ErrorKind,
  msg: String,
}

impl ErrorImpl {
  fn new(kind: ErrorKind, msg: &str) -> ErrorImpl {
    ErrorImpl {
      kind,
      msg: msg.to_string(),
    }
  }
}
