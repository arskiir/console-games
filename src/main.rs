use console_games::{guess_the_word::GuessTheWord, Play};

fn main() {
    println!("press ctrl + c to exit\n");

    loop {
        GuessTheWord::print_intro();
        GuessTheWord::start();
    }
}
