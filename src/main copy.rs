macro_rules! Command {
    ($cmd:ident $subcmd:ident {$($arg:ident: $t:ty),*}, $func:ident) => {
        match (command, subcommand, args) {
            (stringify!($cmd), stringify!($subcmd), args) if args.len() == count_args!($($arg),*) => {
                let mut iter = args.into_iter();
                $(
                    let $arg = iter.next().unwrap().parse::<$t>().unwrap();
                )*
                $func($($arg),*);
            }
            _ => println!("Invalid command"),
        }
    };
}

macro_rules! count_args {
    ($($arg:ident),*) => {
        <[()]>::len(&[$(replace_expr!($arg ())),*])
    };
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
    ($_t:tt) => {
        ()
    };
}

pub fn timer_create(minutes: u64, name: &str) {
    println!("Timer created for {} minutes with name {}", minutes, name);
}

fn main() {
    let command = "timer";
    let subcommand = "create";
    let args = vec!["90".to_string(), "work".to_string()];

    Command!(timer create {minutes: u64, name: &str}, timer_create);
}


// use clap::{App, Arg, SubCommand};

// fn main() {
// let matches = App::new("i6")
//         .subcommand(
//             SubCommand::with_name("timer")
//                 .about("Manages timers")
//                 .arg(
//                     Arg::with_name("minutes")
//                         .index(1)
//                         .takes_value(true),
//                 )
//                 .arg(
//                     Arg::with_name("name")
//                         .index(2)
//                         .takes_value(true),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("create")
//                         .about("Creates a new timer")
//                         .arg(
//                             Arg::with_name("minutes")
//                                 .short('m')
//                                 .long("minutes")
//                                 .takes_value(true)
//                                 .help("Sets the duration of the timer in minutes"),
//                         )
//                         .arg(
//                             Arg::with_name("name")
//                                 .short('n')
//                                 .long("name")
//                                 .takes_value(true)
//                                 .help("Sets the name of the timer"),
//                         ),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("list")
//                         .about("Lists all timers")
//                         .alias("ls"),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("stop")
//                         .about("Stops a timer")
//                         .arg(
//                             Arg::with_name("name")
//                                 .short('n')
//                                 .long("name")
//                                 .takes_value(true)
//                                 .help("Stops the timer with the given name"),
//                         )
//                         .arg(
//                             Arg::with_name("all")
//                                 .short('a')
//                                 .long("all")
//                                 .takes_value(false)
//                                 .help("Stops all timers"),
//                         ),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("history")
//                         .about("Shows the history of all timers")
//                         .alias("log")
//                         .arg(
//                             Arg::with_name("json")
//                                 .short('j')
//                                 .long("json")
//                                 .takes_value(false)
//                                 .help("Prints the history in JSON format"),
//                         ),
//                 ),
//         )
//         .get_matches();


//     if let Some(matches) = matches.subcommand_matches("timer") {
//         if let Some(matches) = matches.subcommand_matches("create") {
//             let minutes = matches.value_of("minutes").unwrap_or_default();
//             let name = matches.value_of("name").unwrap_or_default();
//             i6::timer::create::create_timer(minutes, name);
//         } else if matches.subcommand_matches("list").is_some() {
//             i6::timer::list::list_timers();
//         } else if let Some(matches) = matches.subcommand_matches("stop") {
//             if matches.is_present("all") {
//                 i6::timer::stop::stop_all_timers();
//             } else {
//                 i6::timer::stop::stop_timer(matches.value_of("name").unwrap_or_default());
//             }
//         } else if let Some(matches) = matches.subcommand_matches("history") {
//             if matches.is_present("json") {
//                 i6::timer::print::print_history_json();
//             } else {
//                 i6::timer::print::print_history();
//             }
//         } else if let (Some(minutes), Some(name)) = (matches.value_of("minutes"), matches.value_of("name")) {
//             i6::timer::create::create_timer(minutes, name);
//         } else if let Some(minutes) = matches.value_of("minutes") {
//             i6::timer::create::create_timer(minutes, "");
//         }
//     }
// }

// use clap::{App, Arg, SubCommand};

// fn main() {
// let matches = App::new("i6")
//         .subcommand(
//             SubCommand::with_name("timer")
//                 .about("Manages timers")
//                 .arg(
//                     Arg::with_name("minutes")
//                         .index(1)
//                         .takes_value(true),
//                 )
//                 .arg(
//                     Arg::with_name("name")
//                         .index(2)
//                         .takes_value(true),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("create")
//                         .about("Creates a new timer")
//                         .arg(
//                             Arg::with_name("minutes")
//                                 .short('m')
//                                 .long("minutes")
//                                 .takes_value(true)
//                                 .help("Sets the duration of the timer in minutes"),
//                         )
//                         .arg(
//                             Arg::with_name("name")
//                                 .short('n')
//                                 .long("name")
//                                 .takes_value(true)
//                                 .help("Sets the name of the timer"),
//                         ),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("list")
//                         .about("Lists all timers")
//                         .alias("ls"),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("stop")
//                         .about("Stops a timer")
//                         .arg(
//                             Arg::with_name("name")
//                                 .short('n')
//                                 .long("name")
//                                 .takes_value(true)
//                                 .help("Stops the timer with the given name"),
//                         )
//                         .arg(
//                             Arg::with_name("all")
//                                 .short('a')
//                                 .long("all")
//                                 .takes_value(false)
//                                 .help("Stops all timers"),
//                         ),
//                 )
//                 .subcommand(
//                     SubCommand::with_name("history")
//                         .about("Shows the history of all timers")
//                         .alias("log")
//                         .arg(
//                             Arg::with_name("json")
//                                 .short('j')
//                                 .long("json")
//                                 .takes_value(false)
//                                 .help("Prints the history in JSON format"),
//                         ),
//                 ),
//         )
//         .get_matches();


//     if let Some(matches) = matches.subcommand_matches("timer") {
//         if let Some(matches) = matches.subcommand_matches("create") {
//             let minutes = matches.value_of("minutes").unwrap_or_default();
//             let name = matches.value_of("name").unwrap_or_default();
//             i6::timer::create::create_timer(minutes, name);
//         } else if matches.subcommand_matches("list").is_some() {
//             i6::timer::list::list_timers();
//         } else if let Some(matches) = matches.subcommand_matches("stop") {
//             if matches.is_present("all") {
//                 i6::timer::stop::stop_all_timers();
//             } else {
//                 i6::timer::stop::stop_timer(matches.value_of("name").unwrap_or_default());
//             }
//         } else if let Some(matches) = matches.subcommand_matches("history") {
//             if matches.is_present("json") {
//                 i6::timer::print::print_history_json();
//             } else {
//                 i6::timer::print::print_history();
//             }
//         } else if let (Some(minutes), Some(name)) = (matches.value_of("minutes"), matches.value_of("name")) {
//             i6::timer::create::create_timer(minutes, name);
//         } else if let Some(minutes) = matches.value_of("minutes") {
//             i6::timer::create::create_timer(minutes, "");
//         }
//     }
// }
