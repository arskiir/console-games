use self::internal::PromptDiskMoveResult;
use crate::Play;
use console::{style, Term};
use std::io::{stdin, stdout, Write};
mod internal;

pub struct TowerOfHanoi;

impl TowerOfHanoi {
    fn prompt_disk_count(&self) -> usize {
        let default: usize = 3;
        loop {
            print!("Enter disk count (left empty for default of {default}): ");
            stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim().is_empty() {
                break default;
            }
            let Ok(count) = input.trim().parse() else { continue };
            break count;
        }
    }
}

impl Play for TowerOfHanoi {
    fn name(&self) -> &'static str {
        "Tower of Hanoi"
    }

    fn print_intro(&self) {
        println!("Welcome to {}!\n", style(self.name()).green());
        println!("The objective of the game is to move all disks from the leftmost tower to the rightmost tower.\nYou cannot place a larger disk on top of a smaller disk.\n");
    }

    fn start(&self) {
        let term = Term::stdout();
        let disk_count = self.prompt_disk_count();
        let mut game = internal::TowerOfHanoi::new(disk_count);

        loop {
            term.clear_screen().expect("Failed to clear screen");
            game.render();

            if let Ok(PromptDiskMoveResult { from, to }) = game.prompt_disk_move() {
                if game.move_disk(from, to).is_err() {
                    term.clear_screen().expect("Failed to clear screen");
                    continue;
                }
            } else {
                term.clear_screen().expect("Failed to clear screen");
                continue;
            }

            if game.win() {
                term.clear_screen().expect("Failed to clear screen");
                game.render();
                println!("You win!\n");
                break;
            }
        }
    }
}
