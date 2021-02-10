use project::{Cli, Template};

use console::style;

fn main() {
  let cli = Cli::new();
  let template = Template::from(&cli.args);
  match template.generate() {
    Ok(_) => {
      println!("{}", style("Go to project's directory:").bold());
      if cfg!(unix) {
        println!("\t$ cd {}", cli.args.project.rel_path().display());
        println!("\t$ ls");
      } else {
        println!("\t> cd {}", cli.args.project.rel_path().display());
        println!("\t> dir")
      }
    }
    Err(err) => eprintln!(
      "{} {}",
      style("ERROR:").bold().red(),
      style(err).bold().red()
    ),
  }
}
