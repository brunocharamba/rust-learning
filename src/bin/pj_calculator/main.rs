use std::io::stdin;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum OperationEnum {
  ADD, SUB, MUL, DIV
}

fn main () {

  println!("Select which type of calculator you want:");
  println!("1: Simple Calculator - Type the operation and the two numbers after.");
  println!("2: TODO");
  println!("3: TODO");

  let mut choice: String = String::new();
  stdin()
    .read_line(&mut choice)
    .expect("Error getting the message");

  let choice: i32 = match choice.trim().parse() {
      Ok(num) => num,
      Err(_) => -1,
    };

  match choice {
      1 => simple_calculator(),
      _ => println!("Exited.")
  }

}

fn simple_calculator () {
  println!("Simple Calculator!");

  loop {
      println!("Select the operation:");
      for (index, op) in OperationEnum::iter().enumerate() {
          println!("{}: {:#?}", index + 1, op);
      }

      let mut operation: String = String::new();
      stdin()
        .read_line(&mut operation)
        .expect("Error reading then operation.");

      let operation: OperationEnum = match operation.trim().parse() {
        Ok(num) => match num {
          1 => OperationEnum::ADD,
          2 => OperationEnum::SUB,
          3 => OperationEnum::MUL,
          4 => OperationEnum::DIV,
          _ => {
            println!("Operation do not exist.");
            break;
          }
        },
        Err(_) => {
          println!("Not possible to convert to the operation");
          break;
        }
      };

      // first
      println!("Select the first number: ");
      let mut first: String = String::new();
      stdin().read_line(&mut first).expect("Not possible to convert.");
      let first: f32 = match first.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Problem to parse the value.");
          break;
        }
      };

      // second
      println!("Select the second number: ");
      let mut second: String = String::new();
      stdin().read_line(&mut second).expect("Not possible to convert.");
      let second: f32 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Problem to parse the value.");
          break;
        }
      };

      // do calculation
      match operation {
        OperationEnum::ADD => println!("The result is: {}", first + second),
        OperationEnum::SUB =>  println!("The result is: {}", first - second),
        OperationEnum::MUL =>  println!("The result is: {}", first * second),
        OperationEnum::DIV =>  println!("The result is: {}", first / second),
      }
  }
}