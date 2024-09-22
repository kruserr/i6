use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::extension::BasicConstraints;
use openssl::x509::{X509NameBuilder, X509};
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
    OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path)?;

  Ok(file)
}

pub fn generate_openssl_x509_rsa_cert_key_files(
  cert_file_path: &str,
  key_file_path: &str,
) -> Result<(), Box<dyn Error>> {
  let rsa = Rsa::generate(2048)?;
  let pkey = PKey::from_rsa(rsa)?;

  let mut name = X509NameBuilder::new()?;
  name.append_entry_by_text("CN", "localhost")?;
  let name = name.build();

  let mut builder = X509::builder()?;
  builder.set_version(2)?;
  builder.set_subject_name(&name)?;
  builder.set_issuer_name(&name)?;
  builder.set_pubkey(&pkey)?;
  builder.set_not_before(&*openssl::asn1::Asn1Time::days_from_now(0)?)?;
  builder.set_not_after(&*openssl::asn1::Asn1Time::days_from_now(3650)?)?;

  builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;

  builder.sign(&pkey, MessageDigest::sha256())?;

  let certificate = builder.build();

  let mut key_file = open_or_create_file(key_file_path)?;
  key_file.write_all(&pkey.private_key_to_pem_pkcs8()?)?;

  let mut cert_file = open_or_create_file(cert_file_path)?;
  cert_file.write_all(&certificate.to_pem()?)?;

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
