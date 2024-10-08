use i6_pack::cli;

use clap::{Arg, Command as ClapCommand};
use std::io;

fn main() -> io::Result<()> {
  let matches = ClapCommand::new("i6-pack")
    .version("0.0.1")
    .author("kruserr")
    .about(
      "Compress and encrypt a folder, or decrypt and decompress an archive",
    )
    .arg(
      Arg::new("action")
        .help("Action to perform: pack or unpack")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::new("target")
        .help("Folder to compress and encrypt, or to extract to")
        .required(true)
        .index(2),
    )
    .arg(
      Arg::new("password")
        .help("Password for encryption/decryption")
        .required(true)
        .index(3),
    )
    .get_matches();

  let action = matches.get_one::<String>("action").unwrap();
  let target = matches.get_one::<String>("target").unwrap();
  let password = matches.get_one::<String>("password").unwrap();

  cli::run(action, target, password)
}
