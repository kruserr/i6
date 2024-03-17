pub fn create_timer(minutes: &str, name: &str) {
  // Implement the create timer functionality here
  println!("Creating a timer named '{}' for {} minutes...", name, minutes);
}
#[test]
fn test_create_timer() {
  let minutes = "10";
  let name = "test";
  create_timer(minutes, name);
  // Add assertions here based on your implementation
}
