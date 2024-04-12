use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut attempts = 0;

    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("The secret number was: {}", secret_number);

    loop {
        println!("Please input your guess.");
        attempts += 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :/");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Oh no... so, you're dumb...".red());
                continue;
            }
        };

        let attempts_msg: String = match attempts {
            1..=5 => format!("Congrats. It only took you {} attempts!", attempts),
            6..=10 => format!("Not bad, not bad... {} attempts", attempts),
            _ => format!(
                "Come on! {} attempts. You can do better than that!",
                attempts
            ),
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small :s".red()),
            Ordering::Greater => println!("{}", "Too big :s".red()),
            Ordering::Equal => {
                println!("The secret number was: {}", secret_number);
                println!("{}", attempts_msg);
                println!("{}", "You've won!".green());
                break;
            }
        }
    }
}
