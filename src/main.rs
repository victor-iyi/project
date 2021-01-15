use lotlinx::Cli;

fn main() {
  let cli = Cli::new();
  let config = cli.get_config();
  println!("Project path: {}", config.path.display());
  println!("Verbose: {} | quite: {}", config.verbose, config.quiet);
}
