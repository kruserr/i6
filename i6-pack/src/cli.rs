use crate::utils;
use crate::compression;
use crate::encryption;

pub fn run(action: &str, target: &str, password: &str) -> std::io::Result<()> {
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
      compression::compress_tar_file(tar_file, compressed_file)?;
      encryption::encrypt_file(compressed_file, encrypted_file, password)?;
    }
    "unpack" => {
      encryption::decrypt_file(
        target_path.to_str().unwrap(),
        compressed_file,
        password,
      )?;
      compression::decompress_file(compressed_file, tar_file)?;
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
  std::fs::remove_file(compressed_file)?;

  Ok(())
}
