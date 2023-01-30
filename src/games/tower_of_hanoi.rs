use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use console::Term;

use crate::Play;

const POLE_COUNT: usize = 3;
const MAX_DISK_COUNT: usize = 3;

struct Disk {
    size: usize,
}

struct Pole {
    disks: Vec<Disk>,
}

impl Pole {
    fn build(disks_count: usize) -> Self {
        assert!(disks_count <= MAX_DISK_COUNT);

        let mut disks = Vec::with_capacity(disks_count);
        for size in (1..=disks_count).rev() {
            disks.push(Disk { size });
        }

        Self { disks }
    }
}

pub struct TowerOfHanoi;

struct TowerOfHanoiImpl {
    poles: [Pole; POLE_COUNT],
}

struct PromptDiskMoveResult {
    from: usize,
    to: usize,
}

use std::error;
use std::fmt;

#[derive(Debug)]
struct ParseDiskMoveError {
    message: String,
}

impl ParseDiskMoveError {
    fn new(pole_index: usize) -> Self {
        ParseDiskMoveError {
            message: format!("Pole {} not found", pole_index),
        }
    }
}

impl fmt::Display for ParseDiskMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ParseDiskMoveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl TowerOfHanoiImpl {
    /// print the poles and pole numbers
    fn render(&self) {
        // print to poles
        for i in (0..MAX_DISK_COUNT).rev() {
            for pole in self.poles.iter() {
                if let Some(disk) = pole.disks.get(i) {
                    print!("|{}|", disk.size);
                } else {
                    print!("| |");
                }
            }
            println!();
        }
        // print pole numbers
        for i in 0..POLE_COUNT {
            print!(" {} ", i + 1);
        }
        println!();
    }

    fn prompt_disk_move(&mut self) -> Result<PromptDiskMoveResult, Box<dyn Error>> {
        let mut input = String::new();
        print!("From: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let mut from: usize = input.trim().parse()?;
        from -= 1;
        if self.poles.get(from).is_none() {
            return Err(Box::new(ParseDiskMoveError::new(from)));
        }

        input.clear();

        print!("To: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let mut to: usize = input.trim().parse()?;
        to -= 1;
        if self.poles.get(to).is_none() {
            Err(Box::new(ParseDiskMoveError::new(to)))
        } else {
            Ok(PromptDiskMoveResult { from, to })
        }
    }

    /// move a disk from a pole to another pole
    fn move_disk(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        let poles = self.poles.as_mut();
        if let Some(disk) = poles[from].disks.pop() {
            poles[to].disks.push(disk);
            Ok(())
        } else {
            Err("This pole has no disk")
        }
    }

    fn check_win(&self) -> bool {
        let disks = &self.poles.get(POLE_COUNT - 1).unwrap().disks;
        disks.len() == MAX_DISK_COUNT && disks.windows(2).all(|w| w[0].size > w[1].size)
    }
}

impl Default for TowerOfHanoiImpl {
    fn default() -> Self {
        Self {
            poles: [Pole::build(MAX_DISK_COUNT), Pole::build(0), Pole::build(0)],
        }
    }
}

impl Play for TowerOfHanoi {
    fn name(&self) -> &'static str {
        "Tower of Hanoi"
    }

    fn start(&self) {
        let mut game = TowerOfHanoiImpl::default();
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

            if game.check_win() {
                term.clear_screen().expect("Failed to clear screen");
                game.render();
                println!("You win!\n");
                break;
            }

            term.clear_screen().expect("Failed to clear screen");
        }
    }
}
