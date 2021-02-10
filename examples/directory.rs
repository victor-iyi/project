//! Testing the `std::fs::canonicalize(...)` function.
//!
use std::{env, path::PathBuf};

fn main() {
  match std::fs::canonicalize("src/engine") {
    Ok(p) => println!("Fs can: {}", p.display()),
    Err(err) => eprintln!("Err: {}", err),
  }

  let path = PathBuf::from("../../project/../");
  println!("Path: {} is relative", &path.display());
  println!(
    "`PathBuf` canned: {} is relative",
    &path.canonicalize().unwrap().display()
  );
  if path.is_relative() {
    println!("{} is relative", &path.display());
  } else {
    println!("{} is NOT relative", &path.display());
  }

  let curr = env::current_dir().expect("Cannot get the current directory.");
  println!("Current directory: {}", curr.display());
}
