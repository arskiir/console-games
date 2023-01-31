use std::io::{stdout, Write};
mod cell;
use console::Term;

use crate::util::probability;

use self::cell::Cell;

const COORD_SYMBOLS: [char; 35] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const MINE: char = 'X';
const EMPTY: char = ' ';

pub struct MineSweeper {
    field: Vec<Vec<Cell>>,
}

impl MineSweeper {
    pub fn new(size: usize) -> Self {
        let mut field = Vec::with_capacity(size);

        for _ in 0..size {
            let mut row = Vec::with_capacity(size);
            for _ in 0..size {
                row.push(Cell::new(probability(20)));
            }
            field.push(row);
        }

        Self { field }
    }

    pub fn start(&mut self) {
        let term = Term::stdout();
        loop {
            self.print_field();
            let (x, y) = self.prompt_char_coord();
            let Some((x, y)) = self.find_coord(x, y) else {
                term.clear_screen().expect("Failed to clear screen");
                println!("Invalid coordinates");
                continue;
            };
        }
    }

    fn print_field(&self) {}

    fn prompt_char_coord(&self) -> (char, char) {
        print!("Enter xy: ");
        stdout().flush().expect("Flush failed");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut chars = input.chars();
        let x = chars.next();
        let y = chars.next();
        if x.is_none() || y.is_none() {
            self.prompt_char_coord()
        } else {
            (x.unwrap(), y.unwrap())
        }
    }

    fn find_coord(&self, x: char, y: char) -> Option<(usize, usize)> {
        todo!()
    }
}

pub fn prompt_field_size() -> usize {
    let mut input = String::new();
    print!("Enter field size (9 <= size <= {}): ", COORD_SYMBOLS.len());
    stdout().flush().expect("Flush failed");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let size: usize = input.trim().parse().unwrap_or(9);
    if size < 9 {
        9
    } else if size > COORD_SYMBOLS.len() {
        COORD_SYMBOLS.len()
    } else {
        size
    }
}
