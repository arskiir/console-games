use crate::Play;
use console::Term;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

pub struct GuessTheNumber;

impl Play for GuessTheNumber {
    fn name(&self) -> &'static str {
        "Guess the Number"
    }

    fn start(&self) {
        let mut rng = rand::thread_rng();
        let min = 0;
        let max = 100;
        let random_number = rng.gen_range(min..=max);

        let term = Term::stdout();

        for i in (0..7).rev() {
            print!(
                "Guesses left: {}\nBetween {} and {}, inclusive\nYou Choose: ",
                i + 1,
                min,
                max
            );
            stdout().flush().expect("Failed to flush");

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");

            let input = input.trim();
            if input.is_empty() {
                continue;
            }
            let input: u8 = match input.parse() {
                Ok(val) => val,
                Err(_) => continue,
            };

            term.clear_screen().expect("Failed to clear screen");

            match input.cmp(&random_number) {
                Ordering::Less => println!("{input}, Too low!\n"),
                Ordering::Greater => println!("{input}, Too high!\n"),
                Ordering::Equal => {
                    println!("You win!\n");
                    break;
                }
            }
        }

        println!("You lose!\nThe number was {random_number}\n");
    }
}
