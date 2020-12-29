/*use handlebars::Handlebars;
use std::collections::BTreeMap;

fn main() {
  // create the handlebars registry
  let mut handlebars = Handlebars::new();

  // register the template. The template string will be verified and compiled.
  let source = "hello {{world}}";
  assert!(handlebars.register_template_string("t1", source).is_ok());

  // Prepare some data.
  //
  // The data type should implements `serde::Serialize`
  let mut data = BTreeMap::new();
  data.insert("world".to_string(), "世界!".to_string());
  assert_eq!(handlebars.render("t1", &data).unwrap(), "hello 世界!");
  println!("{:?}", data);
  println!("{:?}", handlebars.render("t1", &data).unwrap());
}
*/
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

use lotlinx::cli::Cli;

fn main() {
  let c = Cli::new();
  c.build();
}
