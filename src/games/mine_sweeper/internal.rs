use std::io::{stdout, Write};
mod cell;
use console::{style, Term};

use crate::util::probability;

use self::cell::Cell;

const COORD_SYMBOLS: [char; 35] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const MINE: &str = "⊛";
const HIDDEN: &str = "◼";
const FLAGGED: &str = "⚑";

pub struct MineSweeper {
    field: Vec<Vec<Cell>>,
    size: usize,
    mines_count: usize,
    revealed_count: usize,
    cell_count: usize,
    placed_flags: usize,
}

impl MineSweeper {
    pub fn new(size: usize) -> Self {
        let mut field = Vec::with_capacity(size);
        let mut mine_locations = vec![vec![false; size]; size];
        let mut mines_count = 0;

        for y in 0..size {
            let mut row = Vec::with_capacity(size);
            for x in 0..size {
                let is_mine = probability(20);
                if is_mine {
                    mines_count += 1;
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
                    .filter(|(x, y)| mine_locations[*y][*x])
                    .count();
                cell.set_adjacent_count(adjacent_count);
            }
        }

        Self {
            field,
            size,
            mines_count,
            revealed_count: 0,
            cell_count: size * size,
            placed_flags: 0,
        }
    }

    pub fn start(&mut self) {
        let term = Term::stdout();
        loop {
            term.clear_screen().expect("Failed to clear screen");
            self.show_remaining_flags();
            self.print_field();

            let (x, y, flag) = self.prompt_char_coord();
            let Some((x, y)) = self.find_coord_indices(x, y) else {
                term.clear_screen().expect("Failed to clear screen");
                println!("Invalid coordinates");
                continue;
            };

            let cell = &mut self.field[y][x];

            if cell.is_revealed() {
                term.clear_screen().expect("Failed to clear screen");
                continue;
            }

            if flag {
                let flagged = cell.is_flagged_mut();
                if *flagged {
                    *flagged = false;
                    self.placed_flags -= 1;
                } else {
                    *flagged = true;
                    self.placed_flags += 1;
                }
                continue;
            }

            if cell.is_mine() {
                term.clear_screen().expect("Failed to clear screen");
                self.reveal_all();
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
        println!();
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
                        print!("{} ", self.colored_number(cell.adjacent_count()));
                    }
                } else if cell.is_flagged() {
                    print!("{} ", style(FLAGGED).red());
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

    fn colored_number(&self, number: usize) -> console::StyledObject<usize> {
        match number {
            n @ 0 => style(n).hidden(),
            n @ 1 => style(n).blue(),
            n @ 2 => style(n).green(),
            n @ 3 => style(n).red(),
            n @ 4 => style(n).color256(57),
            n @ 5 => style(n).color256(1),
            n @ 6 => style(n).color256(45),
            n @ 7 => style(n).yellow(),
            n @ 8 => style(n).color256(166),
            n @ _ => style(n).hidden(),
        }
    }

    fn prompt_char_coord(&self) -> (char, char, bool) {
        print!("Enter xy: ");
        stdout().flush().expect("Flush failed");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut chars = input.chars();
        let x_or_f = chars.next();
        if x_or_f == Some('f') {
            let x = chars.next();
            let y = chars.next();
            match (x, y, x_or_f) {
                (Some(x), Some(y), Some('f')) => (x, y, true),
                _ => {
                    println!("Invalid coordinates");
                    self.prompt_char_coord()
                }
            }
        } else {
            let y = chars.next();
            match (x_or_f, y) {
                (Some(x), Some(y)) => (x, y, false),
                _ => {
                    println!("Invalid coordinates");
                    self.prompt_char_coord()
                }
            }
        }
    }

    fn find_coord_indices(&self, _x: char, _y: char) -> Option<(usize, usize)> {
        let x = COORD_SYMBOLS.iter().position(|&c| c == _x);
        let y = COORD_SYMBOLS.iter().position(|&c| c == _y);
        match (x, y) {
            (Some(x), Some(y)) if x < self.size && y < self.size => Some((x, y)),
            _ => None,
        }
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

    fn reveal(&mut self, x: usize, y: usize) {
        let cell = &mut self.field[y][x];
        cell.reveal();
        self.revealed_count += 1;

        if cell.adjacent_count() > 0 {
            return;
        }

        Self::get_adjacent_coord(self.size, x, y)
            .into_iter()
            .for_each(|(x, y)| {
                let cell = &mut self.field[y][x];

                if cell.is_revealed() || cell.is_mine() {
                    return;
                }

                if cell.adjacent_count() > 0 {
                    cell.reveal();
                    self.revealed_count += 1;
                    return;
                }

                self.reveal(x, y);
            });
    }

    fn is_won(&self) -> bool {
        self.revealed_count + self.mines_count == self.cell_count
    }

    fn show_remaining_flags(&self) {
        println!("Remaining flags: {}", self.mines_count - self.placed_flags);
    }

    fn reveal_all(&mut self) {
        self.field.iter_mut().flatten().for_each(Cell::reveal);
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
