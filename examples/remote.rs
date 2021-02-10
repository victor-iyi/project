//! Test using a remote template.
//!
use project::{ProjectInfo, Template, TemplateOptions};

use console::style;

fn main() {
  let project = ProjectInfo::from("./my-project");
  let options =
    TemplateOptions::new("https://github.com/victor-iyi/project", None);
  /* -- OR -- */
  // let options = TemplateOptions::new("victor-iyi/project", None);

  let template = Template::new(&project, &options);
  match template.generate() {
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
