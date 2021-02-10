#![allow(dead_code)]
//! Utility functions for path handling.
//!
//! - `basename` - Returns the basename of a given path (as `&str`).
//! - `filename` - Returns the filename of a path.
//! - `diff_paths` - Renturns the relative path given two paths.
//!
use std::path::{Component, Path, PathBuf};

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
pub fn basename(path: &str) -> &str {
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
/// # use project::util::filename;
///
/// # fn main() {
///
/// assert_eq!(filename(&"foo/bar"), "bar");
/// assert_eq!(filename(&"foo/bar/"), "bar");
/// assert_eq!(filename(&"baz/foo.bar"), "foo.bar");
///
/// # }
/// ```
pub fn filename(path: &dyn AsRef<Path>) -> &str {
  path.as_ref().file_name().and_then(|s| s.to_str()).unwrap()
}

/// Construct a relative path from a provided base directory path to the provided path.
///
/// This routine is adapted from the *old* Path's `path_relative_from`
/// function, which works differently from the new `relative_from` function.
/// In particular, this handles the case on unix where both paths are
/// absolute but with only the root as the common directory.
///
/// Adapted from [librust_back].
///
/// # Example
///
/// ```rust
/// # use project::util::diff_paths;
/// # use std::path::PathBuf;
///
/// # fn main() {
/// let baz: PathBuf = "/foo/bar/baz".into();
/// let bar: PathBuf = "/foo/bar".into();
/// let quux: PathBuf = "/foo/bar/quux".into();
///
/// assert_eq!(diff_paths(&bar, &baz), Some("../".into()));
/// assert_eq!(diff_paths(&baz, &bar), Some("baz".into()));
/// assert_eq!(diff_paths(&quux, &baz), Some("../quux".into()));
/// assert_eq!(diff_paths(&baz, &quux), Some("../baz".into()));
/// assert_eq!(diff_paths(&bar, &quux), Some("../".into()));
/// # }
/// ```
///
/// [librust_back]: https://github.com/rust-lang/rust/blob/e1d0de82cc40b666b88d4a6d2c9dcbc81d7ed27f/src/librustc_back/rpath.rs#L116-L158
///
pub fn diff_paths(path: &Path, base: &Path) -> Option<PathBuf> {
  if path.is_absolute() != base.is_absolute() {
    if path.is_absolute() {
      Some(PathBuf::from(path))
    } else {
      None
    }
  } else {
    let mut ita = path.components();
    let mut itb = base.components();
    let mut comps: Vec<Component> = vec![];
    loop {
      match (ita.next(), itb.next()) {
        (None, None) => break,
        (Some(a), None) => {
          comps.push(a);
          comps.extend(ita.by_ref());
          break;
        }
        (None, _) => comps.push(Component::ParentDir),
        (Some(a), Some(b)) if comps.is_empty() && a == b => (),
        (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
        (Some(_), Some(b)) if b == Component::ParentDir => return None,
        (Some(a), Some(_)) => {
          comps.push(Component::ParentDir);
          for _ in itb {
            comps.push(Component::ParentDir);
          }
          comps.push(a);
          comps.extend(ita.by_ref());
          break;
        }
      }
    }
    Some(comps.iter().map(|c| c.as_os_str()).collect())
  }
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

  #[test]
  #[allow(clippy::blacklisted_name)]
  fn test_diff_paths() {
    let baz: PathBuf = "/foo/bar/baz".into();
    let bar: PathBuf = "/foo/bar".into();
    let quux: PathBuf = "/foo/bar/quux".into();

    assert_eq!(diff_paths(&bar, &baz), Some("../".into()));
    assert_eq!(diff_paths(&baz, &bar), Some("baz".into()));
    assert_eq!(diff_paths(&quux, &baz), Some("../quux".into()));
    assert_eq!(diff_paths(&baz, &quux), Some("../baz".into()));
    assert_eq!(diff_paths(&bar, &quux), Some("../".into()));
  }
}
