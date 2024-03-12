#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn timer_create(minutes: u64, name: String) {
    println!("Timer created for {} minutes with name {}", minutes, name);
}

pub fn timer_list(name: String) {
    println!("Listing timers with name {}", name);
}

enum Command {
    TimerCreate,
    TimerList,
}

lazy_static! {
    static ref COMMANDS: Arc<Mutex<HashMap<String, Command>>> = Arc::new(Mutex::new(HashMap::new()));
}

fn command_insert(key: String, command: Command) {
    let mut commands = COMMANDS.lock().unwrap();
    commands.insert(key, command);
}

fn command_parse(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command_name = if parts.len() > 2 { parts[0..2].join(" ") } else { parts[0].to_string() };
    let commands = COMMANDS.lock().unwrap();
    match commands.get(&command_name) {
        Some(Command::TimerCreate) => {
            let (minutes, name) = if parts.len() > 2 { (parts[2].parse().unwrap(), parts[3].to_string()) } else { (parts[1].parse().unwrap(), parts[2].to_string()) };
            timer_create(minutes, name);
        }
        Some(Command::TimerList) => {
            let name = parts[2].to_string();
            timer_list(name);
        }
        _ => {
            if parts.len() == 3 && command_name == "timer" {
                let minutes: u64 = parts[1].parse().unwrap();
                let name = parts[2].to_string();
                timer_create(minutes, name);
            } else {
                println!("Unknown command {}", command_name);
            }
        }
    }
}

fn main() {
    command_insert("timer".to_string(), Command::TimerCreate);
    command_insert("timer create".to_string(), Command::TimerCreate);
    command_insert("timer list".to_string(), Command::TimerList);
    command_parse("timer create 90 work");
    command_parse("timer 90 work");
}
