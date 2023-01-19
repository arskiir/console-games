pub mod guess_the_word;
mod util;

pub trait Play {
    fn print_intro();
    fn start();
}
