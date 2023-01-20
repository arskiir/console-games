# console games

A hobby project for console games. New games are coming, hopefully 👉😌👉.

## Usage

```rust
use console_games::game_manager::GameCenter;

fn main() {
    GameCenter::enter();
}
```

## Games

List of available games:

[x] Guess the Word
[x] Guess the Number

### To run an individual game

```rust
use console_games::{games::guess_the_word::GuessTheWord, Play};

fn main() {
    println!("{}", GuessTheWord.name());
    GuessTheWord.print_intro();
    GuessTheWord.start();
}
```
