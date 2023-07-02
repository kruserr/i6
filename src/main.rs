use clap::{Args, Parser, Subcommand};

trait IRunCommand {
  fn run(&self);
}

#[derive(Args, Default)]
struct TimerArgs {
  /// Create a new timer with given minutes
  minutes: u32,
}
impl IRunCommand for TimerArgs {
  fn run(&self) {
    println!("{:?}", self.minutes);
  }
}
impl std::fmt::Display for TimerArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "TimerArgs")
  }
}
impl PartialEq for TimerArgs {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

#[derive(Subcommand, PartialEq)]
enum Commands {
  /// Create, list, stop and log timers
  Timer(TimerArgs),
}
impl IRunCommand for Commands {
  fn run(&self) {
    match self {
      Commands::Timer(x) => x.run(),
    }
  }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

struct RunCommandFactory {
  commands: Vec<Commands>,
}
impl RunCommandFactory {
  fn new() -> Self {
    Self {commands: vec![
      Commands::Timer(Default::default()),
    ]}
  }
}

fn main() {
  let cli = Cli::parse();
  let factory = RunCommandFactory::new();

  for command in factory.commands {
    if command == cli.command {
      cli.command.run();
    }
  }
}

#[test]
fn verify_cli() {
  use clap::CommandFactory;
  Cli::command().debug_assert();
}
