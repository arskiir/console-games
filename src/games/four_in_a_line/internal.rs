use console::Term;
use std::io::stdin;

pub(super) struct FourInALine {
    pub(super) table: Table,
    pub(super) turn_of: char,
    pub(super) dropped_count: usize,
    pub(super) term: Term,
}

type Table = [[char; 7]; 6];
const EMPTY: char = '_';
const PLAYER_O: char = 'O';
const PLAYER_X: char = 'X';

impl Default for FourInALine {
    fn default() -> Self {
        Self {
            table: [[EMPTY; 7]; 6],
            turn_of: PLAYER_O,
            dropped_count: 0,
            term: Term::stdout(),
        }
    }
}

impl FourInALine {
    pub(super) fn print_table(&self) {
        for row in self.table.iter() {
            print!("|");
            for spot in row {
                print!(" {spot} |");
            }
            println!();
        }
        for i in 1..=self.col_count() {
            print!("  {i} ");
        }
        println!();
    }

    pub(super) fn col_count(&self) -> usize {
        self.table[0].len()
    }

    pub(super) fn change_turn(&mut self) {
        self.turn_of = if self.turn_of == PLAYER_O {
            PLAYER_X
        } else {
            PLAYER_O
        }
    }

    pub(super) fn get_col_input(&self) -> Option<usize> {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        if input.is_empty() {
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

    pub(super) fn is_col_ok(&self, col: usize) -> bool {
        col < self.col_count() && self.table[0][col] == EMPTY
    }

    pub(super) fn drop_in_col(&mut self, col: usize) -> usize {
        for (row_idx, row) in &mut self.table.iter_mut().enumerate().rev() {
            let spot = &mut row[col];
            if *spot == EMPTY {
                *spot = self.turn_of;
                self.dropped_count += 1;
                return row_idx;
            }
        }

        unreachable!("col {col} should be validated first");
    }

    pub(super) fn get_winner(&self, row_idx: usize, col: usize) -> Option<char> {
        let mut consecutive: u8 = 0;
        // check row
        for spot in &self.table[row_idx] {
            if let Some(value) = self.check_consecutive(spot, &mut consecutive) {
                return value;
            }
        }

        consecutive = 0;
        // check col
        for spot in &self.table.map(|row| row[col]) {
            if let Some(result) = self.check_consecutive(spot, &mut consecutive) {
                return result;
            }
        }

        // check bottom left to top right diagonal
        consecutive = 0;
        if let Some(value) = self.check_bl_to_tr(row_idx, col, &mut consecutive) {
            return value;
        }

        // check top left to bottom right diagonal
        consecutive = 0;
        if let Some(value) = self.check_tl_to_br(row_idx, col, consecutive) {
            return value;
        }

        None
    }

    pub(super) fn check_tl_to_br(
        &self,
        row_idx: usize,
        col: usize,
        mut consecutive: u8,
    ) -> Option<Option<char>> {
        let mut row_idx_it = row_idx as i8;
        let mut col_idx_it = col as i8;

        // walk down first
        loop {
            if row_idx_it == self.table.len() as i8 || col_idx_it == self.col_count() as i8 {
                break;
            }
            let spot = &self.table[row_idx_it as usize][col_idx_it as usize];
            if *spot == EMPTY {
                break;
            }
            if let Some(result) = self.check_consecutive(spot, &mut consecutive) {
                return Some(result);
            }
            row_idx_it += 1;
            col_idx_it += 1;
        }
        let mut row_idx_it = row_idx as i8 - 1;
        let mut col_idx_it = col as i8 - 1;

        // continue walk up if possible
        loop {
            if row_idx_it == -1 || col_idx_it == -1 {
                break;
            }
            let spot = &self.table[row_idx_it as usize][col_idx_it as usize];
            if *spot == EMPTY {
                break;
            }
            if let Some(result) = self.check_consecutive(spot, &mut consecutive) {
                return Some(result);
            }
            row_idx_it -= 1;
            col_idx_it -= 1;
        }
        None
    }

    pub(super) fn check_bl_to_tr(
        &self,
        row_idx: usize,
        col: usize,
        consecutive: &mut u8,
    ) -> Option<Option<char>> {
        let mut row_idx_it = row_idx as i8;
        let mut col_idx_it = col as i8;

        // walk up first
        loop {
            if row_idx_it == -1 || col_idx_it == self.col_count() as i8 {
                break;
            }
            let spot = &self.table[row_idx_it as usize][col_idx_it as usize];
            if *spot == EMPTY {
                break;
            }
            if let Some(result) = self.check_consecutive(spot, consecutive) {
                return Some(result);
            }
            row_idx_it -= 1;
            col_idx_it += 1;
        }
        let mut row_idx_it = row_idx as i8 + 1;
        let mut col_idx_it = col as i8 - 1;

        // continue walk down if possible
        loop {
            if row_idx_it == self.table.len() as i8 || col_idx_it < 0 {
                break;
            }
            let spot = &self.table[row_idx_it as usize][col_idx_it as usize];
            if *spot == EMPTY {
                break;
            }
            if let Some(result) = self.check_consecutive(spot, consecutive) {
                return Some(result);
            }
            row_idx_it += 1;
            col_idx_it -= 1;
        }
        None
    }

    pub(super) fn check_consecutive(
        &self,
        spot: &char,
        consecutive: &mut u8,
    ) -> Option<Option<char>> {
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

    pub(super) fn clear_screen(&mut self) {
        self.term.clear_screen().expect("Failed to clear screen");
    }
}
