use regex::Regex;

fn example_use_regexp() -> () {
  let re = Regex::new("[A-Z][a-z]* *= *'\\d*' *;").unwrap();
  let line = "Value   =  '234'  ; ";
  if let Some(r) = re.find(line) {
    println!("Yes: {}", r.as_str())
  } else {
    println!("No")
  }
}

fn example_use_keyword_ref() -> () {
  let tuple = (String::from("Hello"), String::from("world"));
  match tuple {
    /*
    Без использования ref будет ошибка, так как значение
    будет перемещенно в блок, где происходит декомпозиция
    */
    (ref a, ref b) => {
      println!("a: {}", a);
      println!("b: {}", b);
    }
  }
  println!("({}, {})", tuple.0, tuple.1);
}

fn example_use_slice_for_array() -> () {
  let mut array = [6, 7, 8, 9, 10, 11, 12];
  let array_slice = &mut array[1..5];
  for value in array_slice {
    *value *= 2;
  }

  let mut string = String::from("<<");
  for value in array {
    string.push_str(format!("{}, ", value).as_str());
  }
  string.push_str(">>");

  println!("{}", string);
}

fn example_use_slice_for_vector() -> () {
  let vector = vec![1, 2, 3, 4, 5];

  let range = 0..5;
  let slice = &vector[range];

  for value in slice {
    println!("Значение: {}", value);
  }
}

static mut EXAMPLE_INDEX: usize = 0;

fn print_example(name: &str, callback: impl FnOnce()) {
  unsafe {
    let index = EXAMPLE_INDEX;
    EXAMPLE_INDEX += 1;

    println!("Example[{}] {{{{ >> [{}]\n", index, name);
    callback();
    print!("\n}}}} << [{}]\n\n\n", name);
  }
}

fn main() {
  print_example("Regex", example_use_regexp);
  print_example("Keyword ref", example_use_keyword_ref);
  print_example("Slice for array", example_use_slice_for_array);
  print_example("Slice for vector", example_use_slice_for_vector);
}
