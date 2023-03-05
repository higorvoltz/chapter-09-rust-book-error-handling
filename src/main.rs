use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
  // panic!("crash and burn");

//   $ cargo run
//    Compiling panic v0.1.0 (file:///projects/panic)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.25s
//      Running `target/debug/panic`
// thread 'main' panicked at 'crash and burn', src/main.rs:2:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  // let v = vec![1, 2, 3];
  // v[99];

//   $ cargo run
//    Compiling panic v0.1.0 (file:///projects/panic)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.27s
//      Running `target/debug/panic`
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  // let file_result = File::open("hello.txt");

  // let file = match file_result {
  //   Ok(file) => file,
  //   Err(error) => panic!("we could not open this file: {:?}", error),
  // }; //build ok

  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("problem with creating this file: {:?}", e),
      },
      other_error => {
        panic!("problem with opening the file: {:?}", other_error);
      }
    },
  };

  let another_file = File::open("another.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("another.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
  });

//  let greeting_file_v2 = File::open("greeting.txt").unwrap();

//   Finished dev [unoptimized + debuginfo] target(s) in 0.05s
//   Running `target/debug/chapter_09_error_handling`
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:56:53
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  let greeting_file_v2 = File::open("hello.txt").unwrap(); // build withou errors


// Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
  let greeting_file_v3 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

  // fn read_username_from_file() -> Result<String, io::Error> {
  //   let mut username_file = File::open("hello.txt")?;
  //   let mut username = String::new();
  //   username_file.read_to_string(&mut username)?;
  //   Ok(username)
  // }

  // fn read_username_from_file() -> Result<String, io::Error> {
  //   let mut username = String::new();
  //   File::open("hello.txt")?.read_to_string(&mut username)?;

  //   Ok(username)
  // }

  // fn read_username_from_file() -> Result<String, io::Error> {
  //   fs::read_to_string("hello.txt")
  // }

  // read_username_from_file();

}
