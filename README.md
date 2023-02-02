# console games

A hobby project for console games. New games are coming, hopefully 🙂🙂🙂.

Or you want to write some Rust? Help me out by adding a game of your choice!!!  
See [Contribution](#contribution) section for more details.

## Games

List of available games:

- Guess the Word
- Guess the Number
- Word Type
- Four in A Line
- ~~Tower of Hanoi~~ fixing
- Minesweeper

## Usage

```bash
cargo install console-games
```

then run

```bash
console-games
```

### Or as a library

```rust
use console_games::GameCenter;

fn main() {
    GameCenter::enter();
}

```

### To run an individual game

```rust
use console_games::{games::GuessTheWord, Play};

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

    fn start(&self) {
        // create the internal game instance local to this method
        let game = MyGameImpl::new();
        game.start();
    }
}

struct MyGameImpl {
    // --- snip ---
}

// --- snip ---
```

Lastly, make the game visible in the module tree.

```rust
// games.rs

// --- snip ---
mod my_game;
pub use my_game::*;
```

### To add a new game to the game center

Add the game to the return value of `GameCenter::games` method.

```rust
// game_center.rs

// --- snip ---

impl GameCenter {
    // --- snip ---

    pub fn games() -> [Box<dyn Play>; 5 /* <-- update this number */] {
        [
            Box::new(GuessTheWord),
            Box::new(GuessTheNumber),
            Box::new(WordType),
            // -- snip --
            Box::new(MyGame), // <-- add this line
        ]
    }

    // --- snip ---
}
```
