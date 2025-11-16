// Importing io from the standard library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Detail this thread is local to the current execution context
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Please input your guess.");

        // Creating a mutable String variable to store the user's guess
        // Without mut, we would not be able to change the value of guess
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert guess from String to i32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catch-all pattern that matches any value
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("The secret number is: {}", secret_number);
}
