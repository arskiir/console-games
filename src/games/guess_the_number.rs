use crate::Play;
use rand::Rng;
use std::io::{stdin, stdout, Write};

pub struct GuessTheNumber;

impl Play for GuessTheNumber {
    fn name(&self) -> &'static str {
        "Guess the Number"
    }

    fn start(&mut self) {
        let mut rng = rand::thread_rng();
        let min = 0;
        let max = 100;
        let random_number = rng.gen_range(min..=max);

        for guess_left in (0..=7).rev() {
            print!("Guesses left: {guess_left}\nBetween {min} and {max}, inclusive\nYou Choose: ");
            stdout().flush().expect("Failed to flush");

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");

            let input = input.trim();
            if input == "" {
                continue;
            }
            let input: u8 = match input.parse() {
                Ok(val) => val,
                Err(_) => continue,
            };

            if input < random_number {
                println!("Too low!\n");
            } else if input > random_number {
                println!("Too high!\n");
            } else {
                println!("You win!\n");
                break;
            }
        }
    }
}