use std::io::{stdin, stdout, Write};

use crate::{games::guess_the_word::GuessTheWord, Play};

pub struct GameManager;

impl GameManager {
    pub fn start() {
        println!("press ctrl + c to exit\n");

        let mut games: Vec<Box<dyn Play>> = vec![Box::new(GuessTheWord)];

        loop {
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
                    continue;
                }
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
}
