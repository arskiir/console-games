use std::io::{stdout, Write};
mod cell;
use console::Term;

use crate::util::probability;

use self::cell::Cell;

const COORD_SYMBOLS: [char; 35] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const MINE: &str = "⊛";
const HIDDEN: &str = "◼";

pub struct MineSweeper {
    field: Vec<Vec<Cell>>,
    size: usize,
}

impl MineSweeper {
    pub fn new(size: usize) -> Self {
        let mut field = Vec::with_capacity(size);
        let mut mine_locations = vec![vec![false; size]; size];

        for y in 0..size {
            let mut row = Vec::with_capacity(size);
            for x in 0..size {
                let is_mine = probability(20);
                if is_mine {
                    mine_locations[y][x] = true;
                }
                row.push(Cell::new(is_mine));
            }
            field.push(row);
        }

        // count adjacent mines
        for (y, row) in field.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if cell.is_mine() {
                    continue;
                }
                let adjacent_count = Self::get_adjacent_coord(size, x, y)
                    .iter()
                    .map(|(x, y)| u8::from(mine_locations[*y][*x]))
                    .sum();
                cell.set_adjacent_count(adjacent_count);
            }
        }

        Self { field, size }
    }

    pub fn start(&mut self) {
        let term = Term::stdout();
        loop {
            self.print_field();
            let (x, y) = self.prompt_char_coord();
            let Some((x, y)) = self.find_coord_indices(x, y) else {
                term.clear_screen().expect("Failed to clear screen");
                println!("Invalid coordinates");
                continue;
            };

            let cell = &self.field[y][x];

            if cell.is_revealed() {
                term.clear_screen().expect("Failed to clear screen");
                continue;
            }

            if cell.is_mine() {
                term.clear_screen().expect("Failed to clear screen");
                self.print_field();
                println!("You lose!");
                break;
            }

            self.reveal(x, y);

            if self.is_won() {
                term.clear_screen().expect("Failed to clear screen");
                self.print_field();
                println!("You win!");
                break;
            }
        }
    }

    fn print_field(&self) {
        for (_y, row) in self.field.iter().enumerate() {
            // print y coord symbol
            print!("{} ", COORD_SYMBOLS[_y]);
            for (_x, cell) in row.iter().enumerate() {
                if cell.is_revealed() {
                    if cell.is_mine() {
                        print!("{} ", MINE);
                    } else {
                        print!("{} ", cell.adjacent_count());
                    }
                } else {
                    print!("{} ", HIDDEN);
                }
            }
            println!();
        }
        // print x coord symbols
        print!("  ");
        for x in 0..self.size {
            print!("{} ", COORD_SYMBOLS[x]);
        }
        println!();
    }

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

    fn find_coord_indices(&self, _x: char, _y: char) -> Option<(usize, usize)> {
        todo!()
    }

    fn get_adjacent_coord(size: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        [
            (x as i32 - 1, y as i32 - 1),
            (x as i32, y as i32 - 1),
            (x as i32 + 1, y as i32 - 1),
            (x as i32 - 1, y as i32),
            (x as i32 + 1, y as i32),
            (x as i32 - 1, y as i32 + 1),
            (x as i32, y as i32 + 1),
            (x as i32 + 1, y as i32 + 1),
        ]
        .iter()
        .filter_map(|(x, y)| {
            if *x < 0 || *y < 0 {
                return None;
            }
            let x = *x as usize;
            let y = *y as usize;
            if x >= size || y >= size {
                return None;
            }
            Some((x, y))
        })
        .collect()
    }

    fn reveal(&self, _x: usize, _y: usize) {
        todo!()
    }

    fn is_won(&self) -> bool {
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
