pub trait Encryption {
  fn encrypt_file(
    input_file: &str,
    output_file: &str,
    password: &str,
  ) -> std::io::Result<()>;

  fn decrypt_file(
    input_file: &str,
    output_file: &str,
    password: &str,
  ) -> std::io::Result<()>;
}
