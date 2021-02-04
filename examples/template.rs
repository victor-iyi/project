use project::{Cli, Template};

fn main() {
  let cli = Cli::new();
  let template = Template::from(&cli.args);
  match template.generate() {
    Ok(_) => println!("Success!"),
    Err(err) => eprintln!("ERROR: {}", err),
  }
}
