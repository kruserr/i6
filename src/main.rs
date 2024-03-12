use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

type CommandFn = fn(Vec<&str>);

lazy_static! {
    static ref COMMANDS: Mutex<HashMap<String, CommandFn>> = Mutex::new(HashMap::new());
}

fn command_insert(commands: Vec<&str>, f: CommandFn) {
    let mut command_map = COMMANDS.lock().unwrap();
    for command in commands {
        command_map.insert(command.to_string(), f);
    }
}

fn command_parse(command: &str) {
    let command_map = COMMANDS.lock().unwrap();
    let parts: Vec<&str> = command.split_whitespace().collect();
    let command = parts[0];
    let args = parts[1..].to_vec();

    if let Some(&f) = command_map.get(&format!("{} {}", parts[0], parts[1])) {
        f(parts[2..].to_vec());
    } else if let Some(&f) = command_map.get(command) {
        f(args);
    }
}

fn timer_create(args: Vec<&str>) {
    // Implement your timer creation logic here
    println!("Creating timer with args: {:?}", args);
}

fn timer_list(args: Vec<&str>) {
    // Implement your timer listing logic here
    println!("Listing timers with args: {:?}", args);
}

fn main() {
    command_insert(vec!["timer"], timer_create);
    command_insert(vec!["timer create"], timer_create);
    command_insert(vec!["timer list"], timer_list);

    command_parse("timer create 90 work"); // should create a 90 minute timer with name work
    command_parse("timer 90 work"); // should also create a 90 minute timer with name work
    command_parse("timer 90"); // should create a 90 minute timer with no name
    command_parse("timer list work"); // should list timer with name work
    command_parse("timer list"); // should list all timers
}
