use std::io::{stdin, stdout, Write};

use crate::{
    games::{guess_the_number::GuessTheNumber, guess_the_word::GuessTheWord},
    Play,
};

pub struct GameManager;

impl GameManager {
    /// call this function to start the console game application
    pub fn start() {
        println!("press ctrl + c to exit\n");

        let mut games: [Box<dyn Play>; 2] = [Box::new(GuessTheWord), Box::new(GuessTheNumber)];

        loop {
            let (game_idx_err_msg, game_idx) = match Self::select_game(&games) {
                Some(value) => value,
                None => continue,
            };
            println!();

            match games.get_mut(game_idx) {
                Some(game) => {
                    game.print_intro();
                    game.start();
                }
                None => println!("{}", &game_idx_err_msg),
            };
        }
    }

    fn select_game(games: &[Box<dyn Play>]) -> Option<(String, usize)> {
        println!("Select your game");
        for (i, game) in games.iter().enumerate() {
            println!("{}: {}", i, game.name())
        }
        print!("Game number: ");
        stdout().flush().expect("Flush failed");

        let mut game_idx = String::new();
        stdin()
            .read_line(&mut game_idx)
            .expect("Cannot read game number");
        let game_idx_err_msg = format!(
            "Game number must be an integer between {} to {}",
            0,
            games.len() - 1,
        );
        let game_idx: usize = match game_idx.trim_end().parse() {
            Ok(idx) => idx,
            Err(_) => {
                println!("{}", &game_idx_err_msg);
                return None;
            }
        };

        Some((game_idx_err_msg, game_idx))
    }
}
