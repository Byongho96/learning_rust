use rand::Rng;
use std::cmp::Ordering; // Enum type for comparison
use std::io; // standard io library

// impl Rng for ThreadRng {
// }

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng() returns `ThreadRng` instance

    // println! is a macro that prints a string to the screen
    println!("Guess the number between 1 and 100!");

    let mut cnt = 0;

    loop {
        cnt += 1;

        println!("Please input your guess.");

        // bind empty string instance to mutable variable guess
        let mut guess = String::new(); // ::new() is an associated function of the `String`` type

        // std::io::stdin() is also the same expression.
        io::stdin()
            .read_line(&mut guess) // append user input to guess,
            // The & indicates that this argument is a `reference`.
            // References are also "immutable" by default in Rust. Thus add &mut
            .expect("Failed to read line"); // `read_line` returns `io::Result`.
                                            // `io::Result` is an `enumeration`, which is a type that can be in one of multiple possible states. (Ok or Err)
                                            // `expect()` panics the program and prints the message if the Result is an `Err` value.

        // Shadowing the previous guess variable
        // Enum type can be handled by `match` expression
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed {} times", cnt);
                break; // exit loop
            }
        }
    }
}
