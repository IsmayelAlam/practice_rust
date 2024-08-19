use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("----------------------------");
        println!("Please input your guess:");
        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read the number, please try again.");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        print!("Your guessed {guess}: ");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win 🥳");
                break;
            }
        }
    }
}
