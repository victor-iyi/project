use lotlinx::Cli;

fn main() {
  let cli = Cli::new();
  let config = cli.get_config();
  println!("Project name: {}", config.name);
  println!("Verbose: {} | quite: {}", config.verbose, config.quiet);
}
