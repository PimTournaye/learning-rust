use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Variables are IMMUTABLE by default in Rust, adding mut makes it mutable / changeable.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");

}