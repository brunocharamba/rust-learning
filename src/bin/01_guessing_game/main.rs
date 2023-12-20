use std::io;
use rand::Rng;
use std::cmp;

fn main() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number...");
                continue;
            },
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            },
            cmp::Ordering::Greater => println!("Your number is greater..."),
            cmp::Ordering::Less => println!("Your number is lower..."),
        }
    }
}
