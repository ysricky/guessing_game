use colored::*;
use names::Generator;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut score = 100;

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut player_name = Generator::default();

    let player_name = player_name.next().unwrap();

    loop {
        println!("__________________________________");
        println!("Hello {}, please input your guess.", player_name.yellow());

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!(
                    "{}, {}! Score: {}",
                    "You win".green(),
                    player_name.yellow(),
                    score.to_string().cyan()
                );
                break;
            }
        }

        score = score - 10;
    }
}
