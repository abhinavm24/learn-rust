// Import external crate for random number generation
use rand::Rng;
use rust_book_examples::print_chapter_header;
// Ordering is an enum with variants: Less, Greater, Equal
use std::cmp::Ordering;
// Standard library for input/output operations
use std::io;

fn main() {
    print_chapter_header("Chapter 2", "Programming a Guessing Game");
    
    println!("Guess the number!");

    // Generate a random number between 1 and 100 (inclusive)
    // thread_rng() gives us a random number generator local to the current thread
    // gen_range(1..=100) creates a range from 1 to 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Start an infinite loop - we'll break out when the user wins
    loop {
        println!("Please input your guess.");

        // Create a new, empty String to store user input
        // 'mut' makes it mutable so we can modify it
        let mut guess = String::new();

        // Read user input from stdin
        // read_line returns a Result<usize, Error>
        // expect() will panic with the given message if there's an error
        // The &mut tells Rust we're borrowing 'guess' mutably
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the string into an unsigned 32-bit integer
        // This is "variable shadowing" - we're creating a new variable with the same name
        // trim() removes whitespace/newlines, parse() converts string to number
        // match handles the Result<u32, ParseIntError> returned by parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,        // If parsing succeeded, use the number
            Err(_) => continue,    // If parsing failed, skip to next iteration
        };

        println!("You guessed: {guess};");

        // Compare the guess with secret_number
        // cmp() returns an Ordering enum variant
        // We match on all possible variants
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Secret num is: {secret_number}");
                println!("You win!");
                break;  // Exit the loop when user wins
            }
        }
    }
}
