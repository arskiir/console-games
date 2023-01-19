pub mod games;
mod util;

/// The main trait to classify a struct as a playable game.
pub trait Play {
    fn print_intro();
    fn start();
}
