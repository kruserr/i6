fn command_timer() -> clap::Command {
  clap::Command::new("timer")
    .about("A timer")
    .subcommand_required(true)
    .subcommands(vec![
      clap::Command::new("create").about("Create a timer").args(vec![
        clap::Arg::new("minutes")
          .help("set the minutes for the timer")
          .num_args(1)
          .value_parser(clap::value_parser!(u32).range(1..))
          .required(true),
        clap::Arg::new("minutes")
          .short('m')
          .long("minutes")
          .help("set the minutes for the timer")
          .num_args(1)
          .value_parser(clap::value_parser!(u32).range(1..))
          .required(true),
        clap::Arg::new("name")
          .short('n')
          .long("name")
          .help("set the name of the timer")
          .num_args(1)
          .value_parser(clap::value_parser!(String))
          .required(false),
      ]),
      clap::Command::new("list").about("List timers"),
      clap::Command::new("stop").about("Stop timers"),
      clap::Command::new("log").about("Show timer logs").alias("history"),
    ])
}

fn command_factory() -> Vec<clap::Command> {
  vec![command_timer()]
}

fn command_root() -> clap::Command {
  clap::command!()
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommands(command_factory())
}

fn main() {
  let matches = command_root().get_matches();

  println!("{:?}", matches.subcommand());
}

// #[test]
// fn verify_cmd() {
//   command_root().debug_assert();
// }
