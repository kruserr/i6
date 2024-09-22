use std::borrow::Cow::{self, Borrowed, Owned};
use std::collections::HashMap;

pub mod command;
pub mod lang;

use chrono::{Local, Timelike};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use whoami::fallible;

use self::command::Command;
use self::lang::{InterpreterTraitObject, LexerTraitObject, ParserTraitObject};

fn to_rfc1035(s: &str) -> String {
  s.chars()
    .enumerate()
    .map(|(i, c)| match c {
      'a'..='z' | 'A'..='Z' | '0'..='9' => c,
      '-' if i != 0 && i != s.len() - 1 => c,
      _ => '-',
    })
    .collect::<String>()
    .to_lowercase()
}

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
  #[rustyline(Completer)]
  completer: FilenameCompleter,
  highlighter: MatchingBracketHighlighter,
  #[rustyline(Validator)]
  validator: MatchingBracketValidator,
  #[rustyline(Hinter)]
  hinter: HistoryHinter,
  colored_prompt: String,
}

impl Highlighter for MyHelper {
  fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
    &'s self,
    prompt: &'p str,
    default: bool,
  ) -> Cow<'b, str> {
    if default {
      Borrowed(&self.colored_prompt)
    } else {
      Borrowed(prompt)
    }
  }

  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    Owned("\x1b[2m".to_owned() + hint + "\x1b[m")
  }

  fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
    self.highlighter.highlight(line, pos)
  }

  fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
    self.highlighter.highlight_char(line, pos, forced)
  }
}

pub struct custom_pwd;
impl Command for custom_pwd {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let args = input.split(" ").collect::<Vec<&str>>();

    let path_buf = std::env::current_dir().unwrap_or_default();
    return Ok(path_buf.to_str().unwrap_or_default().into());
  }
}

pub fn shell_main(
  lexer: impl LexerTraitObject,
  parser: impl ParserTraitObject,
  interpreter: impl InterpreterTraitObject,
) -> rustyline::Result<()> {
  let mut temp: HashMap<String, Box<dyn Command>> = Default::default();

  temp.insert("custom_pwd".to_owned(), Box::new(custom_pwd));

  // return shell_main_with_all(lexer, parser, interpreter, Default::default());
  return shell_main_with_all(lexer, parser, interpreter, temp);
}

// To debug rustyline:
// RUST_LOG=rustyline=debug cargo run --example example 2> debug.log
pub fn shell_main_with_all(
  lexer: impl LexerTraitObject,
  parser: impl ParserTraitObject,
  interpreter: impl InterpreterTraitObject,
  custom_commands: HashMap<String, Box<dyn Command>>,
) -> rustyline::Result<()> {
  // env_logger::init();
  let config = Config::builder()
    .history_ignore_space(true)
    .completion_type(CompletionType::List)
    .edit_mode(EditMode::Vi)
    .build();

  let h = MyHelper {
    completer: FilenameCompleter::new(),
    highlighter: MatchingBracketHighlighter::new(),
    hinter: HistoryHinter::new(),
    colored_prompt: "".to_owned(),
    validator: MatchingBracketValidator::new(),
  };

  let mut rl = Editor::with_config(config)?;
  rl.set_helper(Some(h));

  rl.bind_sequence(KeyEvent::ctrl('o'), Cmd::ClearScreen);

  rl.bind_sequence(KeyEvent::ctrl('l'), Cmd::CompleteHint);
  rl.bind_sequence(KeyEvent::ctrl('j'), Cmd::HistorySearchForward);
  rl.bind_sequence(KeyEvent::ctrl('k'), Cmd::HistorySearchBackward);

  rl.clear_screen()?;

  if rl.load_history("history.txt").is_err() {
    println!("No previous history.");
  }

  loop {
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();

    let username = whoami::username();
    let hostname = to_rfc1035(&fallible::hostname().unwrap_or_default());

    let path_buf = std::env::current_dir().unwrap_or_default();
    let pwd = path_buf.to_str().unwrap_or_default();

    let p = format!(
      "[{hour:02}:{minute:02}:{second:02}] - {username}@{hostname}:{pwd}\n$ "
    );

    rl.helper_mut().expect("No helper").colored_prompt = format!("{p}\x1b[0m");
    let readline = rl.readline(&p);

    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str())?;

        println!("Line: {line}");

        let tokens = lexer.run(&line);
        let ast = parser.run(tokens);
        interpreter.run(ast, &custom_commands);
      }
      Err(ReadlineError::Interrupted) => (),
      Err(ReadlineError::Eof) => {
        break;
      }
      Err(err) => {
        println!("Error: {err:?}");
        // break;
      }
    }
  }

  return rl.append_history("history.txt");
}
