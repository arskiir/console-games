use crate::util::get_char_input;

use std::collections::BTreeSet;

use crate::Play;

pub struct GuessTheWord;

impl Play for GuessTheWord {
    fn start() {
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
            // check for win conditions
            if unique_chars.len() == 0 {
                println!("You win!\nThe word is: {}\n", word);
                break;
            }
            if guess_left == 0 {
                println!("You lose!\nThe word is: {}\n", word);
                break;
            }

            // print the current game state
            for c in word.chars() {
                if guessed_chars.contains(&c) || c == ' ' {
                    print!("{}", c);
                } else {
                    print!("{}", '_');
                }
            }
            println!("\nGuesses left: {guess_left}");
            print!("Pick one: ");
            for c in alphabets {
                if guessed_chars.contains(&c) {
                    print!("{}", '_');
                } else {
                    print!("{}", c);
                }
                print!(" ");
            }
            println!();

            let input = get_char_input();
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
            println!();
        }
    }

    fn print_intro() {
        println!("Welcome to Guess the Word!");
    }
}
