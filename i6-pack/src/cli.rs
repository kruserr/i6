use std::io::Write;
use std::path::PathBuf;

use crate::compression;
use crate::encryptions;
use crate::utils;

use crate::encryptions::encryption::Encryption;

pub fn run(action: &str, target: &str, encrypt: bool) -> std::io::Result<()> {
  let password = &if encrypt {
    print!("Enter password: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let password1 =
      rpassword::read_password().expect("Failed to read password");

    if (action == "pack") {
      print!("Confirm password: ");
      std::io::stdout().flush().expect("Failed to flush stdout");
      let password2 =
        rpassword::read_password().expect("Failed to read password");

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

  return run_non_interactive(
    action,
    target_path.to_str().unwrap_or_default(),
    password,
  );
}

pub fn run_non_interactive(
  action: &str,
  target: &str,
  password: &str,
) -> std::io::Result<()> {
  let encrypt = !password.is_empty();
  let extension = if encrypt { "i6pe" } else { "i6p" };

  let target_path = PathBuf::from(target);

  let tar_file =
    &format!("{}-{}.tar", target_path.display(), uuid::Uuid::new_v4());
  let compressed_file =
    &format!("{}-{}.tar.zst", target_path.display(), uuid::Uuid::new_v4());

  let file_out = &format!("{}.{}", target_path.display(), extension);

  match action {
    "pack" => {
      compression::create_tar_archive(target_path.to_str().unwrap(), tar_file)?;

      if (encrypt) {
        compression::compress_tar_file(tar_file, compressed_file)?;
        encryptions::cha_cha20_poly1305::ChaCha20Poly1305::encrypt_file(
          compressed_file,
          file_out,
          password,
        )?;
      } else {
        compression::compress_tar_file(tar_file, file_out)?;
      }
    }
    "unpack" => {
      if (encrypt) {
        encryptions::cha_cha20_poly1305::ChaCha20Poly1305::decrypt_file(
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
        &utils::remove_extension(
          target_path.to_str().unwrap(),
          &format!(".{extension}"),
        ),
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
