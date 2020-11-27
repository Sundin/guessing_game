use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Variables are immutable by default
        let mut guess = String::new();

        // expect = error handling
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow the original guess variable by casting it to an unsigned int
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        // A match expression is made up of arms. 
        // An arm consists of a pattern and the code that should be run if the value matches that arm’s pattern.
        //
        // This comparison means that Rust can infer that the type of secret_number should be u32 (default is i32)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
