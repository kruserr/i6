use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

lazy_static! {
  static ref COMMAND_STATE: Arc<Mutex<HashMap<&'static str, Vec<u8>>>> =
    Arc::new(Mutex::new(HashMap::new()));
}

pub trait Command: Send {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>>;
}

fn is_path(s: &str, used_args: Vec<&str>) -> bool {
  if (used_args.contains(&s)) {
    return false;
  }

  let path = std::path::Path::new(s);
  return path.components().count() > 0;
}

pub struct cd;
impl Command for cd {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let args = input.split(" ").collect::<Vec<&str>>();

    let deafult_path = &"";
    let mut path = args.get(0).unwrap_or(deafult_path).to_string();

    let _ = COMMAND_STATE.lock().map(|mut lock| {
      if (path == "-") {
        lock.get("cd").map(|bytes| {
          String::from_utf8(bytes.clone()).map(|prev_path| {
            path = prev_path;
          })
        });
      }

      let path_buf = std::env::current_dir().unwrap_or_default();
      let temp_pwd = path_buf.to_str().unwrap_or_default();
      lock.insert("cd", temp_pwd.into());
    });

    std::env::set_current_dir(path).unwrap_or_default();

    return Ok("".into());
  }
}

pub struct ls;
impl Command for ls {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let args = input.split(" ").collect::<Vec<&str>>();

    let mut list_all = false;
    if (args.contains(&"-la")) {
      list_all = true;
    }

    let used_args = vec!["-la"];

    let mut path = ".";
    args.last().map(|x| {
      if (is_path(x, used_args)) {
        path = x;
      }
    });

    let mut items = std::fs::read_dir(path)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, std::io::Error>>()
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

    items.sort_by_key(|dir| !dir.is_dir());

    let mut results = String::new();

    if (list_all) {
      for i in 0..items.len() {
        let metadata = std::fs::metadata(&path)
          .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
        let file_type = metadata.file_type();
        let permissions = metadata.permissions();

        let file_type_str = if file_type.is_dir() { "d" } else { "-" };
        let readonly_str = if permissions.readonly() { "r-" } else { "rw" };

        results.push_str(&format!(
          "{}{}x  {}  {}",
          file_type_str,
          readonly_str,
          metadata.len(),
          items.get(i).unwrap().display()
        ));

        if (i < items.len() - 1) {
          results.push('\n');
        }
      }
    } else {
      for path in items {
        results.push_str(&format!("{} ", path.display()));
      }
    }

    return Ok(results);
  }
}

pub struct pwd;
impl Command for pwd {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let args = input.split(" ").collect::<Vec<&str>>();

    let path_buf = std::env::current_dir().unwrap_or_default();
    return Ok(path_buf.to_str().unwrap_or_default().into());
  }
}

pub struct touch;
impl Command for touch {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let path = std::path::Path::new(input);
    match std::fs::OpenOptions::new().create(true).write(true).open(&path) {
      Ok(file) => {
        let now = std::time::SystemTime::now();
        file
          .set_modified(now)
          .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
      }
      Err(e) => return Err(Box::new(e)),
    }

    return Ok("".into());
  }
}

pub struct mkdir;
impl Command for mkdir {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let path = std::path::Path::new(input);
    std::fs::create_dir_all(&path)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
    Ok("".into())
  }
}

pub struct cat;
impl Command for cat {
  fn run(
    &self,
    input: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send>> {
    let contents = std::fs::read_to_string(input)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
    Ok(contents)
  }
}

pub struct echo;
impl echo {
  fn interpret_escapes(s: &str) -> String {
    let mut output = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
      if c == '\\' {
        match chars.next() {
          Some('a') => output.push('\x07'),
          Some('b') => output.push('\x08'),
          Some('c') => break,
          Some('f') => output.push('\x0C'),
          Some('n') => output.push('\n'),
          Some('r') => output.push('\r'),
          Some('t') => output.push('\t'),
          Some('v') => output.push('\x0B'),
          Some('\\') => output.push('\\'),
          Some('0') => {
            let mut octal = String::new();
            for _ in 0..3 {
              if let Some(octal_digit) = chars.peek() {
                if octal_digit.is_digit(8) {
                  octal.push(chars.next().unwrap());
                } else {
                  break;
                }
              }
            }
            if let Ok(value) = u8::from_str_radix(&octal, 8) {
              output.push(value as char);
            }
          }
          _ => (),
        }
      } else {
        output.push(c);
      }
    }
    output
  }
}
impl Command for echo {
  fn run(
    &self,
    input: &str,
  ) -> std::result::Result<String, Box<dyn std::error::Error + Send>> {
    let mut args =
      shlex::split(input).unwrap_or_else(|| vec![input.to_string()]);
    let mut add_newline = false;
    if let Some(first_arg) = args.first() {
      if first_arg == "-n" {
        add_newline = true;
        args.remove(0);
      }
    }
    let output = args
      .iter()
      .map(|arg| echo::interpret_escapes(arg))
      .collect::<Vec<String>>()
      .join(" ");
    if add_newline {
      Ok(format!("{output}\n"))
    } else {
      Ok(output)
    }
  }
}

pub struct mv;
impl Command for mv {
  fn run(
    &self,
    input: &str,
  ) -> std::result::Result<String, Box<dyn std::error::Error + Send>> {
    let args = shlex::split(input).unwrap_or_else(|| vec![input.to_string()]);
    if args.len() != 2 {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "mv command requires exactly 2 arguments",
      )));
    }
    let (src, dest) = (&args[0], &args[1]);
    match std::fs::rename(src, dest) {
      Ok(_) => Ok(String::new()),
      Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send>),
    }
  }
}

pub struct cp;
impl Command for cp {
  fn run(
    &self,
    input: &str,
  ) -> std::result::Result<String, Box<dyn std::error::Error + Send>> {
    let args = shlex::split(input).unwrap_or_else(|| vec![input.to_string()]);
    let mut preserve = false;
    let args: Vec<String> = args
      .into_iter()
      .filter(|arg| {
        if arg == "-p" {
          preserve = true;
          false
        } else {
          true
        }
      })
      .collect();
    if args.len() != 2 {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "cp command requires exactly 2 arguments",
      )));
    }
    let (src, dest) = (&args[0], &args[1]);
    match std::fs::metadata(src) {
      Ok(metadata) => {
        if metadata.is_dir() {
          let entries = match std::fs::read_dir(src) {
            Ok(entries) => entries,
            Err(e) => {
              return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
            }
          };
          let dest_path = std::path::Path::new(dest);
          match std::fs::create_dir_all(dest_path) {
            Ok(_) => (),
            Err(e) => {
              return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
            }
          };
          for entry in entries {
            let entry = match entry {
              Ok(entry) => entry,
              Err(e) => {
                return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
              }
            };
            let entry_path = entry.path();
            let dest_child_path = match entry_path.file_name() {
              Some(name) => dest_path.join(name),
              None => {
                return Err(Box::new(std::io::Error::new(
                  std::io::ErrorKind::Other,
                  "Invalid file name",
                ))
                  as Box<dyn std::error::Error + Send>)
              }
            };
            match std::fs::copy(&entry_path, &dest_child_path) {
              Ok(_) => (),
              Err(e) => {
                return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
              }
            };
            if preserve {
              let metadata = match std::fs::metadata(&entry_path) {
                Ok(metadata) => metadata,
                Err(e) => {
                  return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
                }
              };
              let permissions = metadata.permissions();
              match std::fs::set_permissions(&dest_child_path, permissions) {
                Ok(_) => (),
                Err(e) => {
                  return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
                }
              };
            }
          }
        } else {
          match std::fs::copy(src, dest) {
            Ok(_) => (),
            Err(e) => {
              return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
            }
          };
          if preserve {
            let metadata = match std::fs::metadata(src) {
              Ok(metadata) => metadata,
              Err(e) => {
                return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
              }
            };
            let permissions = metadata.permissions();
            match std::fs::set_permissions(dest, permissions) {
              Ok(_) => (),
              Err(e) => {
                return Err(Box::new(e) as Box<dyn std::error::Error + Send>)
              }
            };
          }
        }
        Ok(String::new())
      }
      Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send>),
    }
  }
}

pub struct rm;
impl Command for rm {
  fn run(
    &self,
    input: &str,
  ) -> std::result::Result<String, Box<dyn std::error::Error + Send>> {
    let args = shlex::split(input).unwrap_or_else(|| vec![input.to_string()]);
    if args.len() != 1 {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "rm command requires exactly 1 argument",
      )));
    }
    let path = &args[0];
    let metadata = match std::fs::metadata(path) {
      Ok(metadata) => metadata,
      Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send>),
    };
    if metadata.is_dir() {
      match std::fs::remove_dir_all(path) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send>),
      };
    } else {
      match std::fs::remove_file(path) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send>),
      };
    }
    Ok(String::new())
  }
}

pub struct clear;
impl Command for clear {
  fn run(
    &self,
    _input: &str,
  ) -> std::result::Result<String, Box<dyn std::error::Error + Send>> {
    print!("\x1Bc");
    std::io::Write::flush(&mut std::io::stdout())
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
    return Ok("".into());
  }
}

pub struct grep;
impl Command for grep {
  fn run(
    &self,
    input: &str,
  ) -> std::result::Result<String, Box<dyn std::error::Error + Send>> {
    let args = shlex::split(input).unwrap_or_else(|| vec![input.to_string()]);
    if args.len() != 2 {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "grep command requires exactly 2 arguments",
      )));
    }
    let (pattern, text) = (&args[0], &args[1]);
    let mut results = String::new();
    for line in text.lines() {
      if line.contains(pattern) {
        results.push_str(line);
        results.push('\n');
      }
    }
    Ok(results)
  }
}

lazy_static! {
  pub static ref DEFAULT_COMMANDS: Arc<
    Mutex<
      Result<
        HashMap<String, Box<dyn Command>>,
        Box<dyn std::error::Error + Send>,
      >,
    >,
  > = Arc::new(Mutex::new({
    let mut result: HashMap<String, Box<dyn Command>> = Default::default();

    result.insert("cd".to_owned(), Box::new(cd));
    result.insert("ls".to_owned(), Box::new(ls));
    result.insert("pwd".to_owned(), Box::new(pwd));
    result.insert("touch".to_owned(), Box::new(touch));
    result.insert("mkdir".to_owned(), Box::new(mkdir));
    result.insert("cat".to_owned(), Box::new(cat));
    result.insert("echo".to_owned(), Box::new(echo));
    result.insert("mv".to_owned(), Box::new(mv));
    result.insert("cp".to_owned(), Box::new(cp));
    result.insert("rm".to_owned(), Box::new(rm));
    result.insert("clear".to_owned(), Box::new(clear));
    result.insert("grep".to_owned(), Box::new(grep));

    Ok(result)
  }));
}
