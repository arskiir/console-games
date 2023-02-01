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

        for row_mines_loc in mine_locations.iter_mut() {
            let mut row = Vec::with_capacity(size);
            for mine_loc in row_mines_loc.iter_mut() {
                let is_mine = probability(20.0);
                if is_mine {
                    mines_count += 1;
                    *mine_loc = true;
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
            println!();
            self.print_field(None);

            let Some((x, y, flag)) = self.prompt_char_coord() else {
                continue;
            };
            let Some((x, y)) = self.find_coord_indices(x, y) else {
                continue;
            };

            let cell = &mut self.field[y][x];

            if cell.is_revealed() {
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
                self.reveal_all_mines();
                self.print_field(Some((x, y)));
                println!("You lose!");
                break;
            }

            self.reveal(x, y);

            if self.is_won() {
                term.clear_screen().expect("Failed to clear screen");
                self.print_field(None);
                println!("You win!");
                break;
            }
        }
        println!();
    }

    fn print_field(&self, last_coord: Option<(usize, usize)>) {
        for (y, row) in self.field.iter().enumerate() {
            // print y coord symbol
            let y_sym = COORD_SYMBOLS[y];

            for (x, cell) in row.iter().enumerate() {
                let x_sym = COORD_SYMBOLS[x];
                if cell.is_revealed() {
                    if cell.is_mine() {
                        let highlight_mine = if let Some((last_x, last_y)) = last_coord {
                            last_x == x && last_y == y
                        } else {
                            false
                        };
                        print!(
                            " {} ",
                            if highlight_mine {
                                style(MINE).yellow()
                            } else {
                                style(MINE).color256(208)
                            }
                        );
                    } else {
                        print!(" {} ", self.colored_number(cell.adjacent_count()));
                    }
                } else if cell.is_flagged() {
                    print!(" {} ", style(FLAGGED).red());
                } else {
                    print!("{}{} ", x_sym, y_sym);
                }
            }
            println!();
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
            n => style(n).hidden(),
        }
    }

    fn prompt_char_coord(&self) -> Option<(char, char, bool)> {
        print!("Enter xy or fxy: ");
        stdout().flush().expect("Flush failed");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: Vec<char> = input.trim().chars().take(3).collect();
        if input.len() < 2 {
            return None;
        }
        if input.len() == 2 {
            return Some((input[0], input[1], false));
        }
        if input[0] != 'f' {
            return None;
        }
        Some((input[1], input[2], true))
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

    fn reveal_all_mines(&mut self) {
        self.field
            .iter_mut()
            .flatten()
            .filter(|c| c.is_mine())
            .for_each(|c| {
                c.reveal();
            });
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
