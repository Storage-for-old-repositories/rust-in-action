use clap::{arg, Command};

fn main() {
  let matches = Command::new("Example using clap module")
    .version("1.0")
    .author("givename")
    .about("learn 'Rust' programming language")
    .arg(
      arg!(<pattern>)
        .help("The pattern to search for")
        .required(true),
    )
    .get_matches();

  println!("arg: {:?}", matches.get_one::<String>("pattern").expect("required"));
}

// cargo run -- --help
// cargo run -- value
