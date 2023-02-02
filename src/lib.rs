mod game_center;

pub use game_center::*;
pub mod games;
mod util;

/// The main trait to classify a struct as a playable game.
pub trait Play {
    /// returns the name of the game
    fn name(&self) -> &'static str;

    /// optionally returns the instructions of the game
    fn instructions(&self) -> Option<&'static str> {
        None
    }

    /// start the game.
    /// The game state should be exclusively local to this function
    fn start(&self);
}
