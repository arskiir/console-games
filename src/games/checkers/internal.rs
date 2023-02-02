use std::{
    collections::BTreeMap,
    io::{stdin, stdout, Write},
};

use console::style;

#[derive(PartialEq)]
enum Player {
    Math,
    Alphabet,
}

pub enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

const MATH_NAMES: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*'];
const ALPHABET_NAMES: [char; 12] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];

struct Checker {
    king: bool,
    owner: Player,
    name: char,
}

impl Checker {
    pub fn new(owner: Player, name: char) -> Self {
        Self {
            king: false,
            owner,
            name,
        }
    }
}

const BOARD_SIZE: usize = 8;

pub struct Checkers {
    math_locations: BTreeMap<(usize, usize), Checker>,
    alphabet_locations: BTreeMap<(usize, usize), Checker>,
    name_locations: BTreeMap<char, (usize, usize)>,
    turn_of: Player,
}

impl Checkers {
    pub fn new() -> Self {
        let mut math_locations = BTreeMap::new();
        let mut alphabet_locations = BTreeMap::new();
        let mut name_locations = BTreeMap::new();

        // populate the board with checkers
        let mut math_name_iter = MATH_NAMES.into_iter();
        let mut alphabet_name_iter = ALPHABET_NAMES.into_iter();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if y < 3 && (y + x) % 2 == 1 {
                    let name = math_name_iter.next().unwrap();
                    math_locations.insert((x, y), Checker::new(Player::Math, name));
                    name_locations.insert(name, (x, y));
                } else if y > 4 && (y + x) % 2 == 1 {
                    let name = alphabet_name_iter.next().unwrap();
                    alphabet_locations.insert((x, y), Checker::new(Player::Alphabet, name));
                    name_locations.insert(name, (x, y));
                }
            }
        }

        Self {
            math_locations,
            alphabet_locations,
            turn_of: Player::Math,
            name_locations,
        }
    }

    pub fn print_board(&self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if let Some(checker) = self.math_locations.get(&(x, y)) {
                    print!("{}", style(checker.name).on_black().blue().bold());
                } else if let Some(checker) = self.alphabet_locations.get(&(x, y)) {
                    print!("{}", style(checker.name).on_black().red().bold());
                } else {
                    print!("_");
                }
                print!("  ")
            }
            println!();
        }
    }

    pub fn print_turn(&self) {
        match self.turn_of {
            Player::Math => println!("Turn of player MATH"),
            Player::Alphabet => println!("Turn of player ALPHABET"),
        }
    }

    pub fn prompt_checker_name(&self) -> Option<char> {
        print!("Checker name: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().chars().next()
    }

    pub fn prompt_direction(&self) -> Option<Direction> {
        print!("Direction q,e,a,d: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "q" => Some(Direction::UpLeft),
            "e" => Some(Direction::UpRight),
            "a" => Some(Direction::DownLeft),
            "d" => Some(Direction::DownRight),
            _ => None,
        }
    }
}
