/*
use lotlinx::template::guidon::Guidon;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn main() {
  let mut guidon = Guidon::new(PathBuf::from("/Users/victor/dev/template"));

  let mut vars = BTreeMap::new();
  vars.insert("name".to_string(), "pricing".to_string());
  vars.insert("description".to_string(), "Pricing model".to_string());
  vars.insert("engine".to_string(), "tf".to_string());
  vars.insert("bucket".to_string(), "lotlinxdata".to_string());
  vars.insert("runtime".to_string(), "2.1".to_string());
  vars.insert("py_version".to_string(), "3.7".to_string());

  guidon.variables(vars);
  guidon.apply_template("/Users/victor/dev/pricing").unwrap();
}
*/
/*
use lotlinx::cli::Cli;

fn main() {
  let c = Cli::default();
  c.build_default();
}
*/
use std::fs;
use walkdir::WalkDir;

fn main() {
  let walker = WalkDir::new("/Users/victor/dev/template/template").into_iter();
  for entry in walker.filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
  }
}
