use console_games::hangman::Hangman;

fn main() {
    println!("press ctrl + c to exit");

    loop {
        Hangman::start();
    }
}
