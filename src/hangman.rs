use std::io::stdin;

pub struct Hangman;

impl Hangman {
    pub fn start() {
        let word = Self::get_word();
        let alphabets: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut guessed_flags: [bool; 26] = [false; 26];
        let mut guess_left = 5;
        loop {
            let mut input = String::new();
            while input == "" || input.len() > 1 {
                println!("Enter a character: ");
                stdin().read_line(&mut input).expect("Failed to read input");
            }
        }
    }

    fn get_word() -> String {
        "todo random word".to_string()
    }
}
