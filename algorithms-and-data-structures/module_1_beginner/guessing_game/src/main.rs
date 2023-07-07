use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // generate random number
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=50);

    loop {
        // ask for user input
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");

        // convert guess to u32, unsigned 32-bit integer
        // shadowing: reusing the guess variable name
        // parse returns a Result type, which is an enum with variants Ok or Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        // compare guess to secret number
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
