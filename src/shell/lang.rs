use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}};

use crate::shell::command::Command;

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

#[derive(Debug, Clone)]
pub enum ASTNode {
  Command { name: String, args: Vec<ASTNode> },
  Operator { op: String, args: Vec<ASTNode> },
  Argument { value: String },
}

impl Default for ASTNode {
  fn default() -> Self {
    todo!()
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
        "&&" | "||" | "|" | ">" | ">>" | "<" => TokenType::Operator,
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
          root = ASTNode::Operator { op: token.value, args };
        }
        _ => eprintln!("Invalid syntax"),
      }
    }

    root
  }
}

pub trait Interpreter: Default {
  fn _run(
    ast: &ASTNode,
    custom_commands: &HashMap<String, Box<dyn Command>>,
  ) -> Result<String, Box<dyn std::error::Error + Send>>;
}
pub trait InterpreterTraitObject {
  fn run(
    &self,
    ast: &ASTNode,
    custom_commands: &HashMap<String, Box<dyn Command>>,
  ) -> Result<String, Box<dyn std::error::Error + Send>>;
}
impl<T: Interpreter> InterpreterTraitObject for T {
  fn run(
    &self,
    ast: &ASTNode,
    custom_commands: &HashMap<String, Box<dyn Command>>,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    <Self as Interpreter>::_run(ast, custom_commands)
  }
}

#[derive(Default)]
pub struct DefaultInterpreter;
impl Interpreter for DefaultInterpreter {
  fn _run(
    ast: &ASTNode,
    custom_commands: &HashMap<String, Box<dyn Command>>,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    match ast {
      ASTNode::Command { name, args } => {
        println!("{:?}", args);
        // return Ok("".into());

        let args: Vec<String> = args
          .into_iter()
          .map(|arg| {
            if let ASTNode::Argument { value } = arg {
              value.into()
            } else {
              eprintln!("Invalid syntax");
              "".into()
            }
          })
          .collect();

        println!("{:?}", args);

        let args_str = &args.join(" ");

        if let Some(custom_command) = custom_commands.get(name) {
          match custom_command.run(args_str) {
            Ok(output) => return Ok(output),
            Err(e) => return Err(e),
          }
        }

        if let Ok(lock) = crate::shell::command::DEFAULT_COMMANDS.lock() {
          if let Ok(commands) = lock.as_ref() {
            if let Some(command) = commands.get(name) {
              match command.run(args_str) {
                Ok(output) => return Ok(output),
                Err(e) => return Err(e),
              }
            }
          }
        }

        match name.as_str() {
          command => {
            let child = std::process::Command::new(command).args(args).spawn();

            match child {
              Ok(child) => {
                let output =
                  String::from_utf8(child.wait_with_output().unwrap().stdout)
                    .unwrap_or_default();
                return Ok(output);
              }
              Err(e) => {
                eprintln!("{}", e);
                return Err(Box::new(e));
              }
            }
          }
        }
      }
      ASTNode::Operator { op, args } => match op.as_str() {
        "&&" => {
          for arg in args {
            DefaultInterpreter::_run(arg, custom_commands);
          }
          return Ok("".into());
        }
        // "|" => {
        //   let mut prev_output = None;
        //   for arg in args {
        //     if (prev_output.is_some()) {
        //       match arg {
        //         ASTNode::Command { name, args: mut next_args } => {
        //           next_args.push(ASTNode::Argument { value: "".into() });
        //         },
        //         _ => {}
        //       }
        //     }
        //     prev_output = Some(DefaultInterpreter::_run(arg, custom_commands));
        //   }
        //   return Ok("".into());
        // }
        _ => {
          return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unknown operator",
          )));
        }
      },
      _ => {
        return Err(Box::new(std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          "Invalid syntax",
        )));
      }
    }
  }
}
