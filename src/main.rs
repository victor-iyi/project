use project::Cli;

fn main() {
  let cli = Cli::new();
  let args = cli.args;
  println!("Project name: {}", args.project.name);
  println!("Project path: {}", args.project.path.display());
  println!("Verbose: {} | quite: {}", args.verbose, args.quiet);
}
