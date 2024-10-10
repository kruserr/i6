use clap::{value_parser, App, Arg, SubCommand};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let http_id = "http";
  let https_id = "https";
  let pack_id = "pack";
  let unpack_id = "unpack";

  let matches = App::new("i6")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(
      SubCommand::with_name(http_id).about("Start a static http server").arg(
        Arg::with_name("port")
          .index(1)
          .default_value("3030")
          .value_parser(value_parser!(u16)),
      ),
    )
    .subcommand(
      SubCommand::with_name(https_id).about("Start a static https server").arg(
        Arg::with_name("port")
          .index(1)
          .default_value("3030")
          .value_parser(value_parser!(u16)),
      ),
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
    .subcommand(
      SubCommand::with_name(pack_id)
        .about("Compress and encrypt")
        .arg(
          Arg::with_name("target")
            .help("Folder to compress and encrypt, or to extract to")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("encrypt")
            .help("Flag to indicate encryption/decryption")
            .short('e')
            .long("encrypt")
            .takes_value(false),
        ),
    )
    .subcommand(
      SubCommand::with_name(unpack_id)
        .about("Decrypt and decompress")
        .arg(
          Arg::with_name("target")
            .help("Folder to compress and encrypt, or to extract to")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("encrypt")
            .help("Flag to indicate encryption/decryption")
            .short('e')
            .long("encrypt")
            .takes_value(false),
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

  if let Some(matches) = matches.subcommand_matches(pack_id) {
    let target = matches.value_of("target").unwrap();
    let encrypt = matches.is_present("encrypt");

    i6_pack::cli::run("pack", target, encrypt)?;
  }

  if let Some(matches) = matches.subcommand_matches(unpack_id) {
    let target = matches.value_of("target").unwrap();
    let encrypt = matches.is_present("encrypt");

    i6_pack::cli::run("unpack", target, encrypt)?;
  }

  Ok(())
}
