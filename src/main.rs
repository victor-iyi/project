use project::{Cli, Template};

fn main() {
  let cli = Cli::new();
  let template = Template::from(&cli.args);
  match template.generate() {
    Ok(_) => println!("Success!"),
    Err(err) => eprintln!("ERROR: {}", err),
  }
  println!("Project name: {}", cli.args.project.name);
  println!("Project path: {}", cli.args.project.path.display());
  println!("Verbose: {} | quite: {}", cli.args.verbose, cli.args.quiet);
}
