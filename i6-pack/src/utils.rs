use std::io;
use std::path::{Path, PathBuf};

pub fn remove_extension(filename: &str, extension: &str) -> String {
  filename.strip_suffix(extension).unwrap_or(filename).to_owned()
}

pub fn validate_path(path: &str) -> io::Result<PathBuf> {
  let path = Path::new(path);
  if path.exists() {
    Ok(path.to_path_buf())
  } else {
    Err(io::Error::new(io::ErrorKind::NotFound, "Path does not exist"))
  }
}

pub fn sanitize_output_path(output_path: &str) -> io::Result<PathBuf> {
  let path = Path::new(output_path);
  if path.is_absolute()
    && !path
      .components()
      .any(|comp| matches!(comp, std::path::Component::ParentDir))
  {
    Ok(path.to_path_buf())
  } else {
    Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid output path"))
  }
}
