use std::fs;
use std::path::Path;

use serde::Serialize;
use walkdir::WalkDir;

use crate::{Error, ErrorKind, Result};

#[cfg(feature = "git")]
pub mod git;
pub mod guidon;
pub(crate) mod helpers;

pub struct Template<P: AsRef<Path>, D: Serialize> {
  /// Source path where `hbs` template exist.
  pub src_path: P,

  /// Path where template will be created.
  pub dest_path: P,

  /// Data that will be written from `src_path` to `dest_path`.
  pub data: Option<D>,

  /// Source paths to ignore. Will not be included in `dest_path`.
  ignore: Option<Vec<P>>,
}

impl<P: AsRef<Path>, D: Serialize> Template<P, D> {
  /// Create a new `Template<T>` with `data` set to `None`.
  pub fn new(src_path: P, dest_path: P) -> Self {
    Template {
      src_path,
      dest_path,
      data: None,
      ignore: None,
    }
  }

  pub fn with_data(src_path: P, dest_path: P, data: D) -> Self {
    Template {
      src_path,
      dest_path,
      data: Some(data),
      ignore: None,
    }
  }
}

impl<P: AsRef<Path>, D: Serialize> Template<P, D> {
  pub fn generate(&self) -> Result<()> {
    // Check if `src_path` is not a directory.
    if !self.src_path.as_ref().exists() {
      return Err(Error::new(
        ErrorKind::NotFound,
        &format!("{} was not found.", self.src_path.as_ref().display()),
      ));
    }

    // Source path was not a directory.
    if !self.src_path.as_ref().is_dir() {
      return Err(Error::new(
        ErrorKind::NotADirectory,
        &format!("{} is a directory.", self.src_path.as_ref().display()),
      ));
    }
    // Create destination directory.
    fs::create_dir_all(&self.dest_path.as_ref())?;

    // Walk the source directory
    // If it's a directory, create the target directory.
    // if it's a file:
    // check if it ends with hbs: call handlebars and write the processed file to target
    // If no substitution, do a direct copy of file to dest path
    let walker = WalkDir::new(self.src_path.as_ref()).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
      // let dest = self.
      let src_path = entry.path();

      let dest_path = self.dest_path.as_ref().join(src_path);
      if src_path.is_file() {
        if src_path.ends_with("hbs") {
          let _src_template = fs::File::open(src_path)?;
        } else {
          let mut _out_file = fs::File::create(&dest_path)?;
          // hb.render_template_source_to_write(&mut , data, writer)
        }
      } else {
        println!("Neither a file nor a directory.");
      }
    }
    Ok(())
  }

  /// Ignore a single path.
  #[inline]
  pub fn ignore_path(&mut self, path: P) {
    self.ignore.as_mut().unwrap().push(path);
  }

  /// Ignore multiple paths.
  #[inline]
  pub fn ignore_paths(&mut self, paths: &[P])
  where
    P: Clone,
  {
    if self.ignore.is_none() {
      self.ignore = Some(paths.to_vec());
    } else {
      self.ignore.as_mut().unwrap().extend(paths.to_owned());
    }
  }
}
