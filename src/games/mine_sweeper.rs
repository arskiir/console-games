use crate::Play;
use console::{style, Term};
pub struct MineSweeper;
mod internal;

impl Play for MineSweeper {
    fn name(&self) -> &'static str {
        "Mine Sweeper"
    }

    fn start(&self) {
        let size: usize = internal::prompt_field_size();
        Term::stdout().clear_screen().unwrap();
        internal::MineSweeper::new(size).start();
    }

    fn print_intro(&self) {
        println!("Welcome to {}!\n", style(self.name()).green());
        println!("Enter x and y coordinates to reveal a cell. Enter 'f' before the coordinates to flag a cell.\n");
    }
}
