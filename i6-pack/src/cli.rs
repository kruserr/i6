use std::io::Write;

use crate::compression;
use crate::encryption;
use crate::utils;

pub fn run(action: &str, target: &str, encrypt: bool) -> std::io::Result<()> {
  let password = &if encrypt {
    print!("Enter password: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let password1 = rpassword::read_password().expect("Failed to read password");

    if (action == "pack") {
      print!("Confirm password: ");
      std::io::stdout().flush().expect("Failed to flush stdout");
      let password2 = rpassword::read_password().expect("Failed to read password");

      if password1 != password2 {
        eprintln!("Passwords do not match.");
        std::process::exit(1);
      }
    }

    password1
  } else {
    "".to_owned()
  };

  // Validate and sanitize the target path
  let target_path = utils::validate_path(target)
    .or_else(|_| utils::sanitize_output_path(target))
    .expect("Invalid target path");

  let tar_file =
    &format!(".{}_{}.tar", target_path.display(), uuid::Uuid::new_v4());
  let compressed_file =
    &format!(".{}_{}.tar.zst", target_path.display(), uuid::Uuid::new_v4());
  let encrypted_file = &format!("{}.i6p", target_path.display());

  match action {
    "pack" => {
      compression::create_tar_archive(target_path.to_str().unwrap(), tar_file)?;

      if (encrypt) {
        compression::compress_tar_file(tar_file, compressed_file)?;
        encryption::encrypt_file(compressed_file, encrypted_file, password)?;
      } else {
        compression::compress_tar_file(tar_file, encrypted_file)?;
      }
    }
    "unpack" => {
      if (encrypt) {
        encryption::decrypt_file(
          target_path.to_str().unwrap(),
          compressed_file,
          password,
        )?;
        compression::decompress_file(compressed_file, tar_file)?;
      } else {
        compression::decompress_file(target_path.to_str().unwrap(), tar_file)?;
      }

      compression::extract_tar_archive(
        tar_file,
        &utils::remove_extension(target_path.to_str().unwrap(), ".i6p"),
      )?;
    }
    _ => {
      eprintln!("Invalid action. Use 'pack' or 'unpack'.");
      std::process::exit(1);
    }
  }

  // Clean up temporary files
  std::fs::remove_file(tar_file)?;

  if (encrypt) {
    std::fs::remove_file(compressed_file)?;
  }

  Ok(())
}
