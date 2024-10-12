use clap::{value_parser, Arg, Command};
use i6_shell::lang::DefaultInterpreter;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let http_id = "http";
  let https_id = "https";
  let pack_id = "pack";
  let unpack_id = "unpack";
  let sh_id = "sh";

  let matches = Command::new("i6")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(
      Command::new(http_id).about("Start a static http server").arg(
        Arg::new("port")
          .index(1)
          .default_value("3030")
          .value_parser(value_parser!(u16)),
      ),
    )
    .subcommand(
      Command::new(https_id).about("Start a static https server").arg(
        Arg::new("port")
          .index(1)
          .default_value("3030")
          .value_parser(value_parser!(u16)),
      ),
    )
    .subcommand(Command::new(sh_id).about("Start an interactive shell"))
    .subcommand(
      Command::new("timer")
        .about("Manages timers")
        .arg(Arg::new("minutes").index(1).value_parser(value_parser!(String)))
        .arg(Arg::new("name").index(2).value_parser(value_parser!(String)))
        .subcommand(
          Command::new("create")
            .about("Creates a new timer")
            .arg(
              Arg::new("minutes")
                .short('m')
                .long("minutes")
                .value_parser(value_parser!(String))
                .help("Sets the duration of the timer in minutes"),
            )
            .arg(
              Arg::new("name")
                .short('n')
                .long("name")
                .value_parser(value_parser!(String))
                .help("Sets the name of the timer"),
            ),
        )
        .subcommand(Command::new("list").about("Lists all timers").alias("ls"))
        .subcommand(
          Command::new("stop")
            .about("Stops a timer")
            .arg(
              Arg::new("name")
                .short('n')
                .long("name")
                .value_parser(value_parser!(String))
                .help("Stops the timer with the given name"),
            )
            .arg(
              Arg::new("all")
                .short('a')
                .long("all")
                .value_parser(value_parser!(bool))
                .help("Stops all timers"),
            ),
        )
        .subcommand(
          Command::new("history")
            .about("Shows the history of all timers")
            .alias("log")
            .arg(
              Arg::new("json")
                .short('j')
                .long("json")
                .value_parser(value_parser!(bool))
                .help("Prints the history in JSON format"),
            ),
        ),
    )
    .subcommand(
      Command::new(pack_id)
        .about("Compress and encrypt")
        .arg(
          Arg::new("target")
            .help("Folder to compress and encrypt, or to extract to")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::new("encrypt")
            .help("Flag to indicate encryption/decryption")
            .short('e')
            .long("encrypt")
            .action(clap::ArgAction::SetTrue)
            .default_value("false"),
        ),
    )
    .subcommand(
      Command::new(unpack_id)
        .about("Decrypt and decompress")
        .arg(
          Arg::new("target")
            .help("Folder to compress and encrypt, or to extract to")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::new("encrypt")
            .help("Flag to indicate encryption/decryption")
            .short('e')
            .long("encrypt")
            .action(clap::ArgAction::SetTrue)
            .default_value("false"),
        ),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches(http_id) {
    println!("http, {:?}", matches);
    let port = *matches.get_one::<u16>("port").unwrap_or(&3030);
    i6::http::create_server_http(port).await?;
  }

  if let Some(matches) = matches.subcommand_matches(https_id) {
    println!("https, {:?}", matches);
    let port = *matches.get_one::<u16>("port").unwrap_or(&3030);
    i6::http::create_server_https(port).await?;
  }

  if let Some(_matches) = matches.subcommand_matches(sh_id) {
    i6_shell::shell_main(
      i6_shell::lang::DefaultLexer,
      i6_shell::lang::DefaultParser,
      i6_shell::lang::DefaultInterpreter,
    )?;
  }

  if let Some(matches) = matches.subcommand_matches("timer") {
    if let Some(matches) = matches.subcommand_matches("create") {
      let temp_string = &String::new();
      let minutes = matches.get_one::<String>("minutes").unwrap_or(temp_string);
      let name = matches.get_one::<String>("name").unwrap_or(temp_string);
      i6::timer::create::create_timer(minutes, name);
    } else if matches.subcommand_matches("list").is_some() {
      i6::timer::list::list_timers();
    } else if let Some(matches) = matches.subcommand_matches("stop") {
      if matches.contains_id("all") {
        i6::timer::stop::stop_all_timers();
      } else {
        i6::timer::stop::stop_timer(
          matches.get_one::<String>("name").unwrap_or(&"".to_string()),
        );
      }
    } else if let Some(matches) = matches.subcommand_matches("history") {
      if matches.contains_id("json") {
        i6::timer::print::print_history_json();
      } else {
        i6::timer::print::print_history();
      }
    } else if let (Some(minutes), Some(name)) =
      (matches.get_one::<String>("minutes"), matches.get_one::<String>("name"))
    {
      i6::timer::create::create_timer(minutes, name);
    } else if let Some(minutes) = matches.get_one::<String>("minutes") {
      i6::timer::create::create_timer(minutes, "");
    }
  }

  if let Some(matches) = matches.subcommand_matches(pack_id) {
    let target = matches.get_one::<String>("target").unwrap();
    let encrypt = matches.get_flag("encrypt");

    i6_pack::cli::run("pack", target, encrypt)?;
  }

  if let Some(matches) = matches.subcommand_matches(unpack_id) {
    let target = matches.get_one::<String>("target").unwrap();
    let encrypt = matches.get_flag("encrypt");

    i6_pack::cli::run("unpack", target, encrypt)?;
  }

  Ok(())
}
