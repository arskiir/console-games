pub mod game_manager;
pub mod games;
mod util;

/// The main trait to classify a struct as a playable game.
pub trait Play {
    fn name(&self) -> &'static str;
    fn print_intro(&self);
    fn start(&mut self);
}
