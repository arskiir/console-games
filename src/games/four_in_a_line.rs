use std::io::{stdin, stdout, Write};

use crate::Play;

pub struct FourInALine {
    table: Option<Table>,
    turn_of: char,
}

type Table = [[char; 7]; 6];

const EMPTY: char = '_';
const PLAYER_O: char = 'O';
const PLAYER_X: char = 'X';

impl FourInALine {
    pub fn new() -> Self {
        Self {
            table: None,
            turn_of: PLAYER_O,
        }
    }

    fn print_table(&self) {
        for row in self.table.unwrap() {
            print!("|");
            for spot in row {
                print!(" {} |", spot);
            }
            println!();
        }
        for i in 1..=self.col_count().unwrap() {
            print!("  {} ", i);
        }
        println!();
    }

    fn init_table(&mut self) {
        self.table = Some([[EMPTY; 7]; 6]);
    }

    fn col_count(&self) -> Option<usize> {
        match self.table {
            Some(table) => Some(table[0].len()),
            None => None,
        }
    }

    fn change_turn(&mut self) {
        self.turn_of = if self.turn_of == PLAYER_O {
            PLAYER_X
        } else {
            PLAYER_O
        }
    }

    fn get_col_input(&self) -> Option<usize> {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        if input == "" {
            return None;
        }
        match input.trim().parse::<usize>() {
            Ok(col_number) => {
                let col_idx = col_number - 1;
                if self.is_col_ok(col_idx) {
                    Some(col_idx)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn is_col_ok(&self, col: usize) -> bool {
        col < (self.col_count().unwrap() - 1) && self.table.unwrap()[0][col] == EMPTY
    }

    fn drop_in_col(&mut self, col: usize) -> usize {
        if let Some(table) = &mut self.table {
            for (row_idx, row) in table.iter_mut().enumerate().rev() {
                let spot = &mut row[col];
                if *spot == EMPTY {
                    *spot = self.turn_of;
                    return row_idx;
                }
            }
        }
        panic!("{col} should be validated first");
    }

    fn get_winner(&self, row_idx: usize, col: usize) -> Option<char> {
        let mut consecutive: u8 = 0;
        // check row
        for spot in &self.table.unwrap()[row_idx] {
            if let Some(value) = self.check_consecutive_spot(spot, &mut consecutive) {
                return value;
            }
        }

        consecutive = 0;
        // check col
        for spot in &self.table.unwrap().map(|row| row[col]) {
            if let Some(result) = self.check_consecutive_spot(spot, &mut consecutive) {
                return result;
            }
        }

        // check diagonals
        consecutive = 0;
        // upper left to bottom right
        let mut row = row_idx;
        let mut col = col;
        loop {
            if row == 0 || col == 0 {
                break;
            }
            row -= 1;
            col -= 1;
        }
        let mut ul_to_br = Vec::new();
        loop {
            if &row == &self.table.unwrap().len() || col == self.col_count().unwrap() {
                break;
            }
            let spot = &self.table.unwrap()[row][col];
            ul_to_br.push(*spot);

            row += 1;
            col += 1;
        }
        for spot in &ul_to_br {
            if let Some(result) = self.check_consecutive_spot(spot, &mut consecutive) {
                return result;
            }
        }

        consecutive = 0;
        // bottom left to upper right
        // FIXME
        let mut row = row_idx;
        let mut col = col;
        loop {
            if row == (&self.table.unwrap().len() - 1) || col == 0 {
                break;
            }
            row += 1;
            col -= 1;
        }
        let mut bl_to_ur = Vec::new();
        loop {
            if col == self.col_count().unwrap() {
                break;
            }
            println!("{row}, {col}");
            let spot = &self.table.unwrap()[row][col];
            bl_to_ur.push(*spot);

            if row == 0 {
                break;
            }

            row -= 1;
            col += 1;
        }
        for spot in &bl_to_ur {
            if let Some(result) = self.check_consecutive_spot(spot, &mut consecutive) {
                return result;
            }
        }

        None
    }

    fn check_consecutive_spot(&self, spot: &char, consecutive: &mut u8) -> Option<Option<char>> {
        if *spot == self.turn_of {
            *consecutive += 1;
            if *consecutive == 4 {
                return Some(Some(self.turn_of));
            }
        } else {
            *consecutive = 0;
        }
        None
    }
}

impl Play for FourInALine {
    fn name(&self) -> &'static str {
        "Four in A Row"
    }

    fn start(&mut self) {
        self.init_table();
        loop {
            self.print_table();

            print!("Play's {} turn: ", self.turn_of);
            stdout().flush().expect("Failed to flush");

            let col = match self.get_col_input() {
                Some(col) => col,
                None => continue,
            };

            let row_idx = self.drop_in_col(col);

            if let Some(player) = self.get_winner(row_idx, col) {
                println!();
                self.print_table();
                println!("Player {player} wins!\n");
                break;
            }

            self.change_turn();
            println!();
        }
    }
}
