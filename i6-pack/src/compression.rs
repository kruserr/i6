use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use tar::Builder;
use walkdir::WalkDir;
use zstd::stream::{decode_all, encode_all};
use zstd::Encoder;

pub fn create_tar_archive<P: AsRef<Path>>(
  folder: P,
  tar_file: &str,
) -> io::Result<()> {
  let tar_gz = File::create(tar_file)?;
  let mut archive = Builder::new(tar_gz);

  for entry in WalkDir::new(folder) {
    let entry = match entry {
      Ok(e) => e,
      Err(e) => {
        eprintln!("Warning: failed to read directory entry: {}", e);
        continue;
      }
    };
    let path = entry.path();
    if path.is_symlink() {
      match std::fs::read_link(path) {
        Ok(target) => {
          if target.exists() {
            if let Err(e) = archive.append_path_with_name(&target, path) {
              eprintln!("Warning: failed to append symlink {:?}: {}", path, e);
            }
          } else {
            eprintln!(
              "Warning: symlink target {:?} does not exist, skipping",
              target
            );
          }
        }
        Err(e) => {
          eprintln!(
            "Warning: failed to read symlink {:?}, skipping: {}",
            path, e
          );
        }
      }
    } else if path.exists() {
      if let Err(e) = archive.append_path(path) {
        eprintln!("Warning: failed to append path {:?}: {}", path, e);
      }
    } else {
      eprintln!("Warning: file {:?} does not exist, skipping", path);
    }
  }

  Ok(())
}

pub fn extract_tar_archive(tar_file: &str, output_dir: &str) -> io::Result<()> {
  let tar_gz = File::open(tar_file)?;
  let mut archive = tar::Archive::new(tar_gz);

  let mut final_output_dir = output_dir.to_string();
  if Path::new(output_dir).exists() {
    final_output_dir = format!("{}-{}", output_dir, uuid::Uuid::new_v4());
  }

  std::fs::create_dir_all(&final_output_dir)?;
  archive.unpack(&final_output_dir)?;
  Ok(())
}

pub fn compress_tar_file(
  tar_file: &str,
  compressed_file: &str,
) -> io::Result<()> {
  let tar = File::open(tar_file)?;
  let compressed = File::create(compressed_file)?;
  let mut tar_reader = tar;
  let mut compressed_writer = compressed;
  let compression_level = 18;

  let mut zstd = zstd::stream::write::Encoder::new(
    &mut compressed_writer,
    compression_level,
  )?;
  zstd.multithread(num_cpus::get() as u32)?;

  zstd.long_distance_matching(true)?;
  zstd.window_log(31)?;

  io::copy(&mut tar_reader, &mut zstd)?;
  zstd.finish()?;

  Ok(())
}

pub fn decompress_file_v1(
  compressed_file: &str,
  output_file: &str,
) -> io::Result<()> {
  let compressed = File::open(compressed_file)?;
  let decompressed_data = decode_all(compressed)?;
  let mut decompressed = File::create(output_file)?;
  decompressed.write_all(&decompressed_data)?;
  Ok(())
}

pub fn decompress_file(
  compressed_file: &str,
  output_file: &str,
) -> io::Result<()> {
  let compressed = File::open(compressed_file)?;
  let mut compressed_reader = compressed;
  let mut decompressed_writer = File::create(output_file)?;

  let mut zstd = zstd::stream::read::Decoder::new(&mut compressed_reader)?;
  zstd.window_log_max(31)?;

  io::copy(&mut zstd, &mut decompressed_writer)?;

  Ok(())
}
