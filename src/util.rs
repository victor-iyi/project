use std::path::Path;

/// Returns the basename of a given path. Works like Python's
/// `os.path.basename`.
///
/// # Example
///
/// ```rust
/// # use project::util::basename;
///
/// # fn main() {
///
/// assert_eq!(basename("bar"), "bar");
/// assert_eq!(basename("foo/bar"), "bar");
/// assert_eq!(basename("foo/bar/"), "bar");
/// assert_eq!(basename("baz/foo.bar"), "foo.bar");
///
/// # }
/// ```
pub(crate) fn basename(path: &str) -> &str {
  Path::new(path)
    .file_name()
    .and_then(|s| s.to_str())
    .unwrap()
}

// /// Split a given path with `std::path::MAIN_SEPARATOR` and return the first split.
// /// [stackoverflow]: https://codereview.stackexchange.com/questions/98536/extracting-the-last-component-basename-of-a-filesystem-path
// fn first_split(path: &str) -> std::borrow::Cow<str> {
//   first_split_with_sep(path, std::path::MAIN_SEPARATOR)
// }

// /// Split a given path with a given separtor and return the first split.
// ///
// /// [stackoverflow]: https://codereview.stackexchange.com/questions/98536/extracting-the-last-component-basename-of-a-filesystem-path
// fn first_split_with_sep(path: &str, sep: char) -> std::borrow::Cow<str> {
//   let mut pieces = path.rsplit(sep);
//   match pieces.next() {
//     Some(p) => p.into(),
//     None => path.into(),
//   }
// }

/// Get the filename (or dirname) from a given `path`.
///
/// # Example
///
/// ```rust
/// # use project::util::basename;
///
/// # fn main() {
///
/// assert_eq!(filename(&"foo/bar"), "bar");
/// assert_eq!(filename(&"foo/bar/"), "bar");
/// assert_eq!(filename(&"baz/foo.bar"), "foo.bar");
///
/// # }
pub(crate) fn filename(path: &dyn AsRef<Path>) -> &str {
  path.as_ref().file_name().and_then(|s| s.to_str()).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_basename() {
    assert_eq!(basename("bar"), "bar");
    assert_eq!(basename("foo/bar"), "bar");
    assert_eq!(basename("foo/bar/"), "bar");
    assert_eq!(basename("baz/foo.bar"), "foo.bar");
  }

  #[test]
  fn test_filename() {
    assert_eq!(filename(&"foo/bar"), "bar");
    assert_eq!(filename(&"foo/bar/"), "bar");
    assert_eq!(filename(&"baz/foo.bar"), "foo.bar");
  }
}
