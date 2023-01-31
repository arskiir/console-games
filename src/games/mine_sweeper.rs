use crate::Play;

pub struct MineSweeper;
mod internal;

impl Play for MineSweeper {
    fn name(&self) -> &'static str {
        "Mine Sweeper"
    }

    fn start(&self) {
        let size: usize = internal::prompt_field_size();
        internal::MineSweeper::new(size).start();
    }
}
