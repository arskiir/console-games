use crate::Play;
use std::io::{stdout, Write};
pub struct FourInALine;
mod internal;

impl Play for FourInALine {
    fn name(&self) -> &'static str {
        "Four in A Line"
    }

    fn start(&self) {
        let mut game = internal::FourInALine::default();

        loop {
            game.clear_screen();
            game.print_table();

            print!("Play's {} turn: ", game.turn_of);
            stdout().flush().expect("Failed to flush");

            let col = match game.get_col_input() {
                Some(col) => col,
                None => continue,
            };

            let row_idx = game.drop_in_col(col);

            if game.dropped_count == game.table.len() * game.table[0].len() {
                game.clear_screen();
                game.print_table();
                println!("Draw!\n");
                break;
            }

            if let Some(player) = game.get_winner(row_idx, col) {
                game.clear_screen();
                game.print_table();
                println!("Player {player} wins!\n");
                break;
            }

            game.change_turn();
        }
    }
}
