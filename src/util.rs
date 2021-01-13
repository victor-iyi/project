use std::path::Path;

/// Returns the basename of a given path. Works like Python's
/// `os.path.basename`.
pub(crate) fn basename(path: &str) -> &str {
  Path::new(path)
    .file_name()
    .and_then(|s| s.to_str())
    .unwrap()
}

// https://codereview.stackexchange.com/questions/98536/extracting-the-last-component-basename-of-a-filesystem-path
// fn basename(path: &str, sep: char) -> std::borrow::Cow<str> {
//     let pieces = path.rsplit(sep);
//     match pieces.next() {
//         Some(p) => p.into(),
//         None => path.into(),
//     }
// }

/// Get the filename (or dirname) from a given `path`.
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
