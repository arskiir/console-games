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
        internal::Checkers::new().start();
    }
}
