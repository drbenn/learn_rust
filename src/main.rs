use std::io;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    fn guessing_game() {
        println!("Guess the number");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The Secret Number is : {secret_number}");

        loop {
            println!("Please input the guess:");
            let mut guess = String::new();
    
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
    
            // original guess input, but breaks if non-number entered
            // let guess: u32 = guess.trim().parse().expect("Please type a Number!");

            // new guess input that will check number type and continue or retry/continue if non-number entered
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            println!("You guessed: {}", guess);
    
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too Small"),
                Ordering::Greater => println!("Too Big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break; // end the game if guess correctly
                }
            }
        }

    }

    guessing_game();
}
