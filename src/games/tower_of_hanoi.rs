use self::internal::PromptDiskMoveResult;
use crate::Play;
use console::Term;
mod internal;

pub struct TowerOfHanoi;

impl Play for TowerOfHanoi {
    fn name(&self) -> &'static str {
        "Tower of Hanoi"
    }

    fn start(&self) {
        let mut game = internal::TowerOfHanoi::default();
        let term = Term::stdout();

        loop {
            game.render();

            if let Ok(PromptDiskMoveResult { from, to }) = game.prompt_disk_move() {
                if game.move_disk(from, to).is_err() {
                    term.clear_screen().expect("Failed to clear screen");
                    continue;
                }
            } else {
                term.clear_screen().expect("Failed to clear screen");
                continue;
            }

            if game.win() {
                term.clear_screen().expect("Failed to clear screen");
                game.render();
                println!("You win!\n");
                break;
            }

            term.clear_screen().expect("Failed to clear screen");
        }
    }
}
