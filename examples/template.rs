use project::{git, Cli, Template};

fn main() {
  let cli = Cli::new();
  let template = Template::from(cli.args);

  // match tmplt.generate(&"/Users/victor/dev/pricing") {
  //   Ok(_) => println!("Suuccessful"),
  //   Err(err) => eprintln!("Error: {}", err),
  // }
}
