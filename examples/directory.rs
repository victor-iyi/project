use std::{env, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

fn main() {
  //   let walker = WalkDir::new("/Users/victor/dev/template").into_iter();
  //   for entry in walker.filter_entry(|e| !is_venv(e)).filter_map(|e| e.ok()) {
  //     println!("{}", entry.path().display());
  //   }

  match std::fs::canonicalize("src/engine") {
    Ok(p) => println!("Fs can: {}", p.display()),
    Err(err) => eprintln!("Err: {}", err),
  }

  let path = PathBuf::from("../../dev/template/../");
  println!("Path: {} is relative", &path.display());
  println!(
    "PathBuf canned: {} is relative",
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

fn is_venv(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.contains("venv"))
    .unwrap_or(false)
}
