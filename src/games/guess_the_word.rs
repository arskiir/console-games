use crate::{util::get_char_input, Play};
use std::{
    collections::BTreeSet,
    io::{stdout, Write},
};

pub struct GuessTheWord;

impl Play for GuessTheWord {
    fn start(&mut self) {
        let word = eff_wordlist::large::random_word();
        let mut unique_chars = BTreeSet::from_iter(word.chars());
        unique_chars.remove(&' ');
        let mut guessed_chars: Vec<char> = Vec::with_capacity(26);
        let mut guess_left = 10;

        let alphabets: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        loop {
            // print the current game state
            for c in word.chars() {
                if guessed_chars.contains(&c) || c == ' ' {
                    print!("{}", c);
                } else {
                    print!("{}", '_');
                }
            }
            println!("\nGuesses left: {guess_left}");
            print!("From ");
            for c in alphabets {
                if guessed_chars.contains(&c) {
                    print!("{}", '_');
                } else {
                    print!("{}", c);
                }
                print!(" ");
            }
            println!();
            print!("You pick: ");
            stdout().flush().expect("Failed to flush");

            let input = get_char_input();
            // check for empty string
            if input.is_none() {
                println!();
                continue;
            }
            let input = input.unwrap();

            if guessed_chars.contains(&input) {
                println!("You have entered a guessed character\n");
                continue;
            }

            guessed_chars.push(input);
            if unique_chars.contains(&input) {
                unique_chars.remove(&input);
            } else {
                guess_left -= 1;
            }

            // check for win conditions
            if unique_chars.len() == 0 {
                println!("You win!\nThe word is: {}\n", word);
                break;
            }
            if guess_left == 0 {
                println!("You lose!\nThe word is: {}\n", word);
                break;
            }

            println!();
        }
    }

    fn print_intro(&self) {
        println!("Welcome to {}!", self.name());
    }

    fn name(&self) -> &'static str {
        "Guess the Word"
    }
}
