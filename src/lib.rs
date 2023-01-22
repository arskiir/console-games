mod game_center;

use console::style;
pub use game_center::*;
pub mod games;

mod util;

/// The main trait to classify a struct as a playable game.
pub trait Play {
    /// returns the name of the game
    fn name(&self) -> &'static str;

    /// print the game's intro or description before the game starts
    fn print_intro(&self) {
        println!("Welcome to {}!\n", style(self.name()).green());
    }

    /// start the game and should return when the game ends
    fn start(&mut self);
}
