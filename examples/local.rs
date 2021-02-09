use console::style;
use project::{ProjectInfo, Template, TemplateOptions};

fn main() {
  let project = ProjectInfo::from("./my-project");
  let options = TemplateOptions::new("../template-test", None);

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
