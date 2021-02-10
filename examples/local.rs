//! Test using a local template path.
//!
use project::{ProjectInfo, Template, TemplateOptions};

use console::style;

fn main() {
  let project = ProjectInfo::from("./my-project");
  let options = TemplateOptions::new("../project", None);

  let template = Template::new(&project, &options);
  match &template.generate() {
    Ok(_) => {
      println!(
        "{}\n\t{} {}",
        style("✨  Project generated!").bold().green(),
        style("$ cd").bold(),
        style(&project.rel_path().display()).bold()
      )
    }
    Err(err) => eprintln!("Error occured: {}", err),
  }
}
