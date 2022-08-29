use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // immutable variable, since we want this to be constant
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // The program has been set up, let's start the loop of the game itself.
    loop {
    println!("Please input your guess.");

    // Variables are IMMUTABLE by default in Rust, adding mut makes it mutable / changeable.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    // {} is a placeholder for the value of guess, think crab claws holding the value.

    // Shadowing(?) the guess variable with a new one, this time with a different type.
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      // _ is a catchall value, in this case, if the user enters a non-number, it will just return the original guess.
      Err(_) => continue,
  };
    println!("You guessed: {guess}");

    // start infinite loop, with a break point
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Press any key to exit the program.
                
                break;
            }
        }
    }
}
