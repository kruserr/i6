use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use tracing_subscriber::fmt::format::FmtSpan;

use warp::Filter;

pub fn open_or_create_file(path: &str) -> Result<File, Box<dyn Error>> {
  let path = Path::new(path);

  if let Some(parent) = path.parent() {
    std::fs::create_dir_all(parent)?;
  }

  let file =
    OpenOptions::new().read(true).write(true).create(true).open(path)?;

  Ok(file)
}

pub fn generate_openssl_x509_rsa_cert_key_files(
  cert_file_path: &str,
  key_file_path: &str,
) -> Result<(), Box<dyn Error>> {
  use rcgen::generate_simple_self_signed;
  let subject_alt_names = vec!["localhost".to_string()];

  let cert = generate_simple_self_signed(subject_alt_names).unwrap();

  let mut cert_file = open_or_create_file(cert_file_path)?;
  cert_file.write_all(cert.serialize_pem()?.as_bytes())?;

  let mut key_file = open_or_create_file(key_file_path)?;
  key_file.write_all(&cert.serialize_private_key_pem().as_bytes())?;

  return Ok(());
}

pub async fn create_server_http(port: u16) -> Result<(), Box<dyn Error>> {
  tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).init();

  warp::serve(warp::fs::dir(".").with(warp::trace::request()))
    .run(([0, 0, 0, 0], port))
    .await;

  return Ok(());
}

pub async fn create_server_https(port: u16) -> Result<(), Box<dyn Error>> {
  let key_file_path = &format!(
    "{}/d4cd362e-89ef-4267-9e35-4cc8a79b60eb/key.pem",
    std::env::temp_dir().to_str().unwrap_or(".")
  );

  let cert_file_path = &format!(
    "{}/d4cd362e-89ef-4267-9e35-4cc8a79b60eb/cert.pem",
    std::env::temp_dir().to_str().unwrap_or(".")
  );

  generate_openssl_x509_rsa_cert_key_files(cert_file_path, key_file_path)?;

  tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).init();

  warp::serve(warp::fs::dir(".").with(warp::trace::request()))
    .tls()
    .cert_path(cert_file_path)
    .key_path(key_file_path)
    .run(([0, 0, 0, 0], port))
    .await;

  return Ok(());
}
