use std::collections::HashMap;

type CommandFn = fn(Vec<&str>);

struct Command {
    name: String,
    function: CommandFn,
    min_args: usize,
}

pub struct CommandParser {
    commands: HashMap<String, Command>,
}

impl CommandParser {
    pub fn new() -> Self {
        CommandParser {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }

    pub fn parse(&self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let input_command = parts[0..2].join(" ");
        if let Some(command) = self.commands.get(&input_command) {
            if parts.len() - 2 >= command.min_args {
                (command.function)(parts[2..].to_vec());
            } else {
                println!("Not enough arguments for command {}", input_command);
            }
        } else {
            println!("Unknown command {}", input_command);
        }
    }
}

macro_rules! Command {
    ($parser:expr, $cmd:ident $subcmd:ident {$($arg:ident: $t:ty),*}, $func:ident) => {
        $parser.register(Command {
            name: format!("{} {}", stringify!($cmd), stringify!($subcmd)),
            function: $func,
            min_args: count_args!($($arg),*),
        });
    };
}

macro_rules! count_args {
    ($($arg:ident),*) => {
        <[()]>::len(&[$(replace_expr!($arg ())),*])
    };
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
    ($_t:tt) => {
        ()
    };
}

pub fn timer_create(args: Vec<&str>) {
    let minutes: u64 = args[0].parse().unwrap();
    let name = args.get(1).unwrap_or(&"");
    println!("Timer created for {} minutes with name {}", minutes, name);
}

fn main() {
    let mut parser = CommandParser::new();
    Command!(parser, timer create {minutes: u64, name: &str}, timer_create);
    Command!(parser, timer {minutes: u64, name: &str}, timer_create);
    parser.parse("timer create 90 work");
}
