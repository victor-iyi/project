use lotlinx::{git, Cli, Template};

fn main() {
  let cli = Cli::new();
  let g = git::init(&cli.config.path, "master");
  match g {
    Ok(repo) => {
      println!("Working directory: {:?}", repo.workdir());
      println!(".git path: {:?}", repo.path().display());
    }
    Err(e) => eprintln!("Error: {}", e),
  }
  // let tmplt = Template::from(cli);
  // match tmplt.generate(&"/Users/victor/dev/pricing") {
  //   Ok(_) => println!("Suuccessful"),
  //   Err(err) => eprintln!("Error: {}", err),
  // }
}
