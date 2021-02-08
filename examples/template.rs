// use project::{Cli, Template};

// fn main() {
//   let cli = Cli::new();
//   let template = Template::from(&cli.args);
//   match template.generate() {
//     Ok(_) => println!("Success!"),
//     Err(err) => eprintln!("ERROR: {}", err),
//   }
// }

use project::{ProjectInfo, Template, TemplateOptions};

fn main() {
  let project = ProjectInfo::from("/Users/victor/dev/project/my-project");
  let options = TemplateOptions::new(
    "https://gitlab.com/victor-iyi/template-test.git",
    None,
  );

  let template = Template::new(&project, &options);
  match template.generate() {
    Ok(_) => {
      println!("Project generated!\ncd {}", &project.rel_path().display())
    }
    Err(err) => eprintln!("Error occured: {}", err),
  }
}
