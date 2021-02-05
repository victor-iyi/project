use project::{Cli, Template};

use console::style;

fn main() {
  let cli = Cli::new();
  let template = Template::from(&cli.args);
  match template.generate() {
    Ok(_) => {
      println!("{}", style("Go to project's directory:").bold());
      println!("\tcd {}", cli.args.project.rel_path().display());
      println!("\tls");
    }
    Err(err) => eprintln!(
      "{} {}",
      style("ERROR:").bold().red(),
      style(err).bold().red()
    ),
  }
}
