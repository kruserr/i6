use clap::{App, Arg, SubCommand};

fn main() {
  let http_id = "http";
  let https_id = "https";

  let matches = App::new("i6")
    .subcommand(
      SubCommand::with_name(http_id).about("Start a static http server"),
    )
    .subcommand(
      SubCommand::with_name(https_id).about("Start a static https server"),
    )
    .subcommand(
      SubCommand::with_name("timer")
        .about("Manages timers")
        .arg(Arg::with_name("minutes").index(1).takes_value(true))
        .arg(Arg::with_name("name").index(2).takes_value(true))
        .subcommand(
          SubCommand::with_name("create")
            .about("Creates a new timer")
            .arg(
              Arg::with_name("minutes")
                .short('m')
                .long("minutes")
                .takes_value(true)
                .help("Sets the duration of the timer in minutes"),
            )
            .arg(
              Arg::with_name("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .help("Sets the name of the timer"),
            ),
        )
        .subcommand(
          SubCommand::with_name("list").about("Lists all timers").alias("ls"),
        )
        .subcommand(
          SubCommand::with_name("stop")
            .about("Stops a timer")
            .arg(
              Arg::with_name("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .help("Stops the timer with the given name"),
            )
            .arg(
              Arg::with_name("all")
                .short('a')
                .long("all")
                .takes_value(false)
                .help("Stops all timers"),
            ),
        )
        .subcommand(
          SubCommand::with_name("history")
            .about("Shows the history of all timers")
            .alias("log")
            .arg(
              Arg::with_name("json")
                .short('j')
                .long("json")
                .takes_value(false)
                .help("Prints the history in JSON format"),
            ),
        ),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches(http_id) {
    println!("http");
  }

  if let Some(matches) = matches.subcommand_matches(https_id) {
    println!("https");
  }

  if let Some(matches) = matches.subcommand_matches("timer") {
    if let Some(matches) = matches.subcommand_matches("create") {
      let minutes = matches.value_of("minutes").unwrap_or_default();
      let name = matches.value_of("name").unwrap_or_default();
      i6::timer::create::create_timer(minutes, name);
    } else if matches.subcommand_matches("list").is_some() {
      i6::timer::list::list_timers();
    } else if let Some(matches) = matches.subcommand_matches("stop") {
      if matches.is_present("all") {
        i6::timer::stop::stop_all_timers();
      } else {
        i6::timer::stop::stop_timer(
          matches.value_of("name").unwrap_or_default(),
        );
      }
    } else if let Some(matches) = matches.subcommand_matches("history") {
      if matches.is_present("json") {
        i6::timer::print::print_history_json();
      } else {
        i6::timer::print::print_history();
      }
    } else if let (Some(minutes), Some(name)) =
      (matches.value_of("minutes"), matches.value_of("name"))
    {
      i6::timer::create::create_timer(minutes, name);
    } else if let Some(minutes) = matches.value_of("minutes") {
      i6::timer::create::create_timer(minutes, "");
    }
  }
}
