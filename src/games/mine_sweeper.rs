use crate::Play;
use console::Term;
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

    fn instructions(&self) -> Option<&'static str> {
        Some("Enter x and y coordinates to reveal a cell. Enter 'f' before the coordinates to flag a cell.")
    }
}
