use console::Term;

use crate::Play;

pub struct Checkers;

mod internal;

impl Play for Checkers {
    fn name(&self) -> &'static str {
        "Checkers"
    }

    fn instructions(&self) -> Option<&'static str> {
        Some("Enter the name of the checker you want to move.\nEnter direction to move.\n'q' for up left, 'e' for up right, 'a' for down left, 'd' for down right.")
    }

    fn start(&self) {
        let mut game = internal::Checkers::new();
        let term = Term::stdout();

        loop {
            game.print_board();
            println!();
            game.print_turn();
            let Some(name) = game.prompt_checker_name() else {
                continue;
            };
            // game.make_move(mv);
            // if game.is_won() {
            //     term.clear_screen().unwrap();
            //     game.print_board();
            //     game.print_winner();
            //     break;
            term.clear_screen().unwrap();
            // }
        }
    }
}
