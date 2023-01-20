# console games

A hobby project for console games. New games are coming, hopefully 🙂🙂🙂.

Or you want to write some Rust? Help me out by adding a game of your choice!!!  
See [Contribution](#contribution) section for more details.

## Usage

```rust
use console_games::game_center::GameCenter;

fn main() {
    GameCenter::enter();
}

```

## Games

List of available games:

- Guess the Word
- Guess the Number
- Word Type

### To run an individual game

```rust
use console_games::{games::guess_the_word::GuessTheWord, Play};

fn main() {
    println!("{}", GuessTheWord.name());
    GuessTheWord.print_intro();
    GuessTheWord.start();
}
```

## Contribution

I need your help!!! Let's grow this project together. If you have any ideas, wether it's a new game, performance improvements, code refactor/redesign, etc, please open an issue or a pull request.

### To create a game

A game must implement the `Play` trait.

```rust
// games/my_game.rs

pub struct MyGame;

impl Play for MyGame {
    fn name(&self) -> &'static str {
        "My Game"
    }

    fn start(&mut self) {
        println!("Starting my game");
    }
}
```

Lastly, make the game visible in the module tree.

```rust
// games.rs

// --- snip ---
pub mod my_game;
```

### To add a new game to the game center

Add the game to the return value of `GameCenter::games` method.

```rust
// game_center.rs

// --- snip ---

impl GameCenter {
    // --- snip ---

    pub fn games() -> [Box<dyn Play>; 4 /* <-- change this number */] {
        [
            Box::new(guess_the_word::GuessTheWord),
            Box::new(guess_the_number::GuessTheNumber),
            Box::new(word_type::WordType),
            Box::new(my_game::MyGame), // <-- add this line
        ]
    }

    // --- snip ---
}
```
