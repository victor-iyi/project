#[derive(Debug)]
struct ErrorImpl {
  _private: (),
}

impl ErrorImpl {
  fn new() -> ErrorImpl {
    ErrorImpl { _private: () }
  }
}

impl std::fmt::Display for ErrorImpl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error")
  }
}

/// Base error type.
#[derive(Debug)]
pub struct Error {
  err: ErrorImpl,
}

impl Error {
  pub fn new() -> Error {
    Error {
      err: ErrorImpl::new(),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.err)
  }
}
