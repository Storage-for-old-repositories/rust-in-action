use clap::{arg, Arg, ArgAction, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let matches = Command::new("Example loading file utility")
    .version("1.0")
    .author("givename")
    .about("learn 'Rust' programming language")
    .arg(arg!(<filepath>).help("file to be read").required(true))
    .arg(
      Arg::new("new")
        .short('n')
        .long("new")
        .help("enable new version parser")
        .action(ArgAction::SetTrue)
        .required(false),
    )
    .get_matches();

  let filepath = matches
    .get_one::<String>("filepath")
    .expect("\"filepath\" is missing");

  let is_using_new_version = matches.get_flag("new");

  let lines = if is_using_new_version == false {
    old_version(filepath)
  } else {
    let dfile = File::open(filepath).unwrap();
    let reader = BufReader::new(dfile);
    new_version(reader)
  };

  let count_lines = lines.len();
  println!("count lines: {}", count_lines);

  if count_lines > 0 {
    println!("begin line: '{}'", lines[0]);
    println!("end line: '{}'", lines[count_lines - 1]);
  }
}

fn old_version(filepath: &String) -> Vec<String> {
  let dfile = File::open(filepath).unwrap();
  let mut reader = BufReader::new(dfile);
  let mut read_line = || {
    let mut line = String::new();

    let is_readed = match reader.read_line(&mut line) {
      Ok(read_count) => read_count > 0,
      _ => false,
    };
    let copied_line = line.clone();
    return (is_readed, copied_line);
  };

  let mut lines: Vec<String> = Vec::with_capacity(16);
  loop {
    let (is_readed, line) = read_line();
    if is_readed == false {
      break;
    }

    lines.push(line);
  }

  return lines;
}

fn new_version<T: BufRead>(reader: T) -> Vec<String> {
  let lines: Vec<_> = reader
    .lines()
    .map(|line| {
      return line.unwrap();
    })
    .collect();

  return lines;
}
