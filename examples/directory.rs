use walkdir::{DirEntry, WalkDir};

fn main() {
  let walker = WalkDir::new("/Users/victor/dev/template").into_iter();
  for entry in walker.filter_entry(|e| !is_venv(e)).filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
  }
}

fn is_venv(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.contains("venv"))
    .unwrap_or(false)
}
