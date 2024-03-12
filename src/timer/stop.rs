pub fn stop_timer(name: &str) {
    // Implement the stop timer functionality here
    println!("Stopping the timer named '{}'...", name);
}
#[test]
fn test_stop_timer() {
    let name = "test";
    stop_timer(name);
    // Add assertions here based on your implementation
}

pub fn stop_all_timers() {
    // Implement the stop all timers functionality here
    println!("Stopping all timers...");
}
#[test]
fn test_stop_all_timers() {
    stop_all_timers();
    // Add assertions here based on your implementation
}
