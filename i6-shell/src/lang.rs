use std::collections::HashMap;

use crate::command::Command;

#[derive(Debug)]
pub enum TokenType {
  Command,
  Argument,
  Operator,
}

impl Default for TokenType {
  fn default() -> Self {
    todo!()
  }
}

#[derive(Debug, Default)]
pub struct Token {
  token_type: TokenType,
  value: String,
}

#[derive(Debug)]
pub enum ASTNode {
  Command { name: String, args: Vec<ASTNode> },
  Operator { op: String, args: Vec<ASTNode> },
  Argument { value: String },
  Pipe { left: Box<ASTNode>, right: Box<ASTNode> },
}

impl Default for ASTNode {
  fn default() -> Self {
    ASTNode::Argument { value: String::new() }
  }
}

pub trait Lexer: Default {
  fn _run(input: &str) -> Vec<Token>;
}
pub trait LexerTraitObject {
  fn run(&self, input: &str) -> Vec<Token>;
}
impl<T: Lexer> LexerTraitObject for T {
  fn run(&self, input: &str) -> Vec<Token> {
    return <Self as Lexer>::_run(input);
  }
}

#[derive(Default)]
pub struct DefaultLexer;
impl Lexer for DefaultLexer {
  fn _run(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    for word in input.split_whitespace() {
      let token_type = match word {
        "&&" | "||" | "|" | ">" | ">>" => TokenType::Operator,
        _ if tokens.is_empty()
          || matches!(
            tokens.last().unwrap().token_type,
            TokenType::Operator
          ) =>
        {
          TokenType::Command
        }
        _ => TokenType::Argument,
      };

      tokens.push(Token { token_type, value: word.to_string() });
    }

    tokens
  }
}

pub trait Parser: Default {
  fn _run(tokens: Vec<Token>) -> ASTNode;
}
pub trait ParserTraitObject {
  fn run(&self, tokens: Vec<Token>) -> ASTNode;
}
impl<T: Parser> ParserTraitObject for T {
  fn run(&self, tokens: Vec<Token>) -> ASTNode {
    return <Self as Parser>::_run(tokens);
  }
}

#[derive(Default)]
pub struct DefaultParser;
impl Parser for DefaultParser {
  fn _run(tokens: Vec<Token>) -> ASTNode {
    let mut iter = tokens.into_iter().peekable();

    let mut root = match iter.next() {
      Some(Token { token_type: TokenType::Command, value }) => {
        ASTNode::Command { name: value, args: vec![] }
      }
      _ => {
        eprintln!("Invalid syntax");
        return ASTNode::default();
      }
    };

    while let Some(token) = iter.next() {
      match token.token_type {
        TokenType::Argument => {
          if let ASTNode::Command { name, args } = &mut root {
            args.push(ASTNode::Argument { value: token.value });
          }
        }
        TokenType::Operator => {
          let mut args = vec![root];
          while let Some(Token { token_type: TokenType::Command, value }) =
            iter.next()
          {
            let mut command_args = vec![];
            while let Some(Token { token_type: TokenType::Argument, value }) =
              iter.peek()
            {
              command_args.push(ASTNode::Argument { value: value.clone() });
              iter.next();
            }
            args.push(ASTNode::Command { name: value, args: command_args });
            if iter
              .peek()
              .map_or(true, |t| matches!(t.token_type, TokenType::Operator))
            {
              break;
            }
          }
          if token.value == "|" {
            let right = args.pop().unwrap();
            let left = args.pop().unwrap();
            root = ASTNode::Pipe {
              left: Box::new(left),
              right: Box::new(right),
            };
          } else {
            root = ASTNode::Operator { op: token.value, args };
          }
        }
        _ => eprintln!("Invalid syntax"),
      }
    }

    root
  }
}

pub trait Interpreter: Default {
  fn _run(ast: ASTNode, custom_commands: &HashMap<String, Box<dyn Command>>);
}
pub trait InterpreterTraitObject {
  fn run(
    &self,
    ast: ASTNode,
    custom_commands: &HashMap<String, Box<dyn Command>>,
  );
}
impl<T: Interpreter> InterpreterTraitObject for T {
  fn run(
    &self,
    ast: ASTNode,
    custom_commands: &HashMap<String, Box<dyn Command>>,
  ) {
    <Self as Interpreter>::_run(ast, custom_commands);
  }
}

#[derive(Default)]
pub struct DefaultInterpreter;
impl Interpreter for DefaultInterpreter {
  fn _run(ast: ASTNode, custom_commands: &HashMap<String, Box<dyn Command>>) {
    match ast {
      ASTNode::Command { name, args } => {
        let args: Vec<String> = args
          .into_iter()
          .map(|arg| {
            if let ASTNode::Argument { value } = arg {
              value
            } else {
              eprintln!("Invalid syntax");
              "".to_owned()
            }
          })
          .collect();

        let args_str = &args.join(" ");

        if let Some(custom_command) = custom_commands.get(&name) {
          let _ = custom_command
            .run(args_str)
            .map(|output| {
              if !output.is_empty() {
                println!("{output}");
              }
            })
            .map_err(|e| eprintln!("{e}"));

          return;
        }

        if let Ok(lock) = crate::command::DEFAULT_COMMANDS.lock() {
          if let Ok(commands) = lock.as_ref() {
            if let Some(command) = commands.get(&name) {
              let _ = command
                .run(args_str)
                .map(|output| {
                  if !output.is_empty() {
                    println!("{output}");
                  }
                })
                .map_err(|e| eprintln!("{e}"));

              return;
            }
          }
        }

        match name.as_str() {
          command => {
            let child = std::process::Command::new(command).args(args).spawn();

            match child {
              Ok(mut child) => {
                child.wait().unwrap();
              }
              Err(e) => eprintln!("{}", e),
            }
          }
        }
      }
      ASTNode::Operator { op, args } => match op.as_str() {
        "&&" => {
          for arg in args {
            DefaultInterpreter::_run(arg, custom_commands);
          }
        }
        _ => eprintln!("Unknown operator"),
      },
      ASTNode::Pipe { left, right } => {
        let first_command = if let ASTNode::Command { name, args } = *left {
          let args: Vec<String> = args
            .into_iter()
            .map(|arg| {
              if let ASTNode::Argument { value } = arg {
                value
              } else {
                eprintln!("Invalid syntax");
                "".to_owned()
              }
            })
            .collect();

          std::process::Command::new(name)
            .args(args)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start first command")
        } else {
          return;
        };

        let second_command = if let ASTNode::Command { name, args } = *right {
          let args: Vec<String> = args
            .into_iter()
            .map(|arg| {
              if let ASTNode::Argument { value } = arg {
                value
              } else {
                eprintln!("Invalid syntax");
                "".to_owned()
              }
            })
            .collect();

          println!("{:?}", args);

          std::process::Command::new(name)
            .args(args)
            .stdin(first_command.stdout.unwrap())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start second command")
        } else {
          return;
        };

        let output = second_command
            .wait_with_output()
            .expect("Failed to wait on second command");

        println!("{}", String::from_utf8_lossy(&output.stdout));
      }
      _ => eprintln!("Invalid syntax"),
    }
  }
}
