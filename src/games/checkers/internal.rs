use std::{
    collections::BTreeMap,
    io::{stdin, stdout, Write},
    str::FromStr,
};

use console::{style, Term};

#[derive(PartialEq)]
enum Player {
    Math,
    Alphabet,
}

impl Player {
    fn name(&self) -> &'static str {
        match self {
            Self::Math => "Math",
            Self::Alphabet => "Alphabet",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "q" => Ok(Self::UpLeft),
            "e" => Ok(Self::UpRight),
            "a" => Ok(Self::DownLeft),
            "d" => Ok(Self::DownRight),
            _ => Err("Invalid direction. 'q', 'e', 'a', 'd' are the only valid directions."),
        }
    }
}

impl Direction {
    fn symbol(&self) -> char {
        match self {
            Self::UpLeft => 'e',
            Self::UpRight => 'q',
            Self::DownLeft => 'a',
            Self::DownRight => 'd',
        }
    }
}

const MATH_NAMES: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*'];
const ALPHABET_NAMES: [char; 12] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];

struct Checker {
    king: bool,
    owner: Player,
    name: char,
}

impl Checker {
    fn new(owner: Player, name: char) -> Self {
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
    possible_moves: Vec<PossibleMove>,
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
            possible_moves: Vec::with_capacity(4),
        }
    }

    pub fn start(&mut self) {
        let term = Term::stdout();

        loop {
            self.possible_moves.clear();
            self.print_board();
            println!();
            self.print_turn();

            let Some(name) = self.prompt_checker_name() else {
                term.clear_screen().unwrap();
                println!("checker name is required. Try again.");
                continue;
            };
            let Some(pos) = self.find_checker_position(name) else {
                term.clear_screen().unwrap();
                println!("cannot find checker with name '{}'. Try again.", name);
                continue;
            };
            let Some(checker) = self.find_checker(&pos) else {
                term.clear_screen().unwrap();
                println!("cannot find checker at position {:?}. Try again.", pos);
                continue;
            };
            if checker.owner != self.turn_of {
                term.clear_screen().unwrap();
                println!("it is not {}'s turn. Try again.", self.turn_of.name());
                continue;
            };

            self.set_possible_moves(&pos);
            if self.possible_moves.is_empty() {
                term.clear_screen().unwrap();
                println!(
                    "no possible moves for checker at position {:?}. Try again.",
                    pos
                );
                continue;
            }

            term.clear_screen().unwrap();
            self.print_board();
            println!();

            let Ok(_dir) = self.prompt_direction() else {
                term.clear_screen().unwrap();
                println!("invalid direction. Try again.");
                continue;
            };
            // self.make_move(mv);
            // if self.is_won() {
            //     term.clear_screen().unwrap();
            //     self.print_board();
            //     self.print_winner();
            //     break;
            // }
            self.turn_of = match self.turn_of {
                Player::Math => Player::Alphabet,
                Player::Alphabet => Player::Math,
            };

            term.clear_screen().unwrap();
        }
    }

    fn highlight_by_player(&self, player: &Player, text: &str) -> String {
        match player {
            Player::Math => style(text).on_black().blue().bold().to_string(),
            Player::Alphabet => style(text).on_black().red().bold().to_string(),
        }
    }

    fn print_board(&self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if let Some(checker) = self.math_locations.get(&(x, y)) {
                    print!(
                        "{}",
                        self.highlight_by_player(&Player::Math, &checker.name.to_string())
                    );
                } else if let Some(checker) = self.alphabet_locations.get(&(x, y)) {
                    print!(
                        "{}",
                        self.highlight_by_player(&Player::Alphabet, &checker.name.to_string())
                    );
                } else if let Some(pos) =
                    self.possible_moves.iter().find(|mv| mv.final_pos == (x, y))
                {
                    print!("{}", pos.dir.symbol());
                } else if self.is_cell_in_any_path(&(x, y)) {
                    print!("_");
                } else {
                    print!(" ");
                }
                print!("  ")
            }
            println!();
        }
    }

    fn print_turn(&self) {
        println!(
            "Turn of player {}",
            self.highlight_by_player(&self.turn_of, self.turn_of.name())
        );
    }

    fn prompt_checker_name(&self) -> Option<char> {
        print!("Checker name: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().chars().next()
    }

    fn prompt_direction(&self) -> Result<Direction, &'static str> {
        let available_dirs = self
            .possible_moves
            .iter()
            .map(|mv| mv.dir.symbol())
            .collect::<Vec<_>>();
        print!("Available directions ");
        for dir in available_dirs.iter() {
            print!("{}", dir);
        }
        print!(": ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse()
    }

    fn find_checker_position(&self, name: char) -> Option<(usize, usize)> {
        self.name_locations.get(&name).copied()
    }

    fn find_checker(&self, pos: &(usize, usize)) -> Option<&Checker> {
        if self.turn_of == Player::Math {
            self.math_locations.get(pos)
        } else {
            self.alphabet_locations.get(pos)
        }
    }

    fn is_cell_in_any_path(&self, pos: &(usize, usize)) -> bool {
        (pos.0 + pos.1) % 2 == 1 && pos.0 < BOARD_SIZE && pos.1 < BOARD_SIZE
    }

    fn is_cell_empty(&self, pos: &(usize, usize)) -> bool {
        self.math_locations.get(pos).is_none()
            && self.alphabet_locations.get(pos).is_none()
            && self.is_cell_in_any_path(pos)
    }

    fn set_possible_moves(&mut self, pos: &(usize, usize)) {
        let current_player_locations = match self.turn_of {
            Player::Math => &self.math_locations,
            Player::Alphabet => &self.alphabet_locations,
        };
        let enemy_locations = match self.turn_of {
            Player::Math => &self.alphabet_locations,
            Player::Alphabet => &self.math_locations,
        };

        // consider down left direction
        let mut last_possible_move_loc: Option<(usize, usize)> = None;
        let mut dir = Direction::DownLeft;
        let mut y_range = (pos.1 + 1)..BOARD_SIZE;
        let mut x_range = (0..pos.0).rev();

        loop {
            let Some(y) = y_range.next() else {
                break;
            };
            let Some(x) = x_range.next() else {
                break;
            };
            let next_x = x.checked_sub(1);

            let next_cell_pos = match next_x {
                Some(next_x) => self.verify_cell_pos((next_x, y + 1)),
                None => None,
            };

            println!("({x}, {y}), next_cell_pos: {:?}", next_cell_pos);

            let FindPossibleMoveResult { final_pos, stop } = self.find_possible_move_pos(
                enemy_locations,
                x,
                y,
                next_cell_pos,
                current_player_locations,
                &mut last_possible_move_loc,
            );
            if let Some(final_pos) = final_pos {
                self.possible_moves.push(PossibleMove { dir, final_pos });
            }
            if stop {
                break;
            }
        }

        // consider down right direction
        last_possible_move_loc = None;
        dir = Direction::DownRight;
        let mut y_range = (pos.1 + 1)..BOARD_SIZE;
        let mut x_range = (pos.0 + 1)..BOARD_SIZE;

        loop {
            let (Some(y), Some(x)) = (y_range.next(), x_range.next()) else {
                break;
            };
            let next_cell_pos = self.verify_cell_pos((x + 1, y + 1));

            let FindPossibleMoveResult { final_pos, stop } = self.find_possible_move_pos(
                enemy_locations,
                x,
                y,
                next_cell_pos,
                current_player_locations,
                &mut last_possible_move_loc,
            );
            if let Some(final_pos) = final_pos {
                self.possible_moves.push(PossibleMove { dir, final_pos });
            }
            if stop {
                break;
            }
        }

        // consider up left direction
        last_possible_move_loc = None;
        dir = Direction::UpLeft;
        let mut y_range = (0..pos.1).rev();
        let mut x_range = (0..pos.0).rev();

        loop {
            let (Some(y), Some(x)) = (y_range.next(), x_range.next()) else {
                break;
            };

            let next_cell_pos = match (x.checked_sub(1), y.checked_sub(1)) {
                (Some(x), Some(y)) => self.verify_cell_pos((x, y)),
                _ => None,
            };

            let FindPossibleMoveResult { final_pos, stop } = self.find_possible_move_pos(
                enemy_locations,
                x,
                y,
                next_cell_pos,
                current_player_locations,
                &mut last_possible_move_loc,
            );
            if let Some(final_pos) = final_pos {
                self.possible_moves.push(PossibleMove { dir, final_pos });
            }
            if stop {
                break;
            }
        }

        // consider up right direction
        last_possible_move_loc = None;
        dir = Direction::UpRight;
        let mut y_range = (0..pos.1).rev();
        let mut x_range = (pos.0 + 1)..BOARD_SIZE;

        loop {
            let (Some(y), Some(x)) = (y_range.next(), x_range.next()) else {
                break;
            };

            let next_cell_pos = match y.checked_sub(1) {
                Some(next_y) => self.verify_cell_pos((x + 1, next_y)),
                None => None,
            };

            let FindPossibleMoveResult { final_pos, stop } = self.find_possible_move_pos(
                enemy_locations,
                x,
                y,
                next_cell_pos,
                current_player_locations,
                &mut last_possible_move_loc,
            );
            if let Some(final_pos) = final_pos {
                self.possible_moves.push(PossibleMove { dir, final_pos });
            }
            if stop {
                break;
            }
        }
    }

    fn find_possible_move_pos(
        &self,
        enemy_locations: &BTreeMap<(usize, usize), Checker>,
        x: usize,
        y: usize,
        next_cell_pos: Option<(usize, usize)>,
        current_player_locations: &BTreeMap<(usize, usize), Checker>,
        last_possible_move_loc: &mut Option<(usize, usize)>,
    ) -> FindPossibleMoveResult {
        if enemy_locations.get(&(x, y)).is_some() {
            // encountered an enemy checker
            // check if there is a checker in the next cell that the current checker can jump over
            println!("hey");
            let Some(next_cell_pos) = next_cell_pos else {
                // there is no next cell
                return FindPossibleMoveResult {final_pos: *last_possible_move_loc, stop: true};
            };

            println!("yo");
            if self.is_cell_empty(&next_cell_pos) {
                return FindPossibleMoveResult {
                    final_pos: Some(next_cell_pos),
                    stop: true,
                };
            }
            println!("usss");

            return FindPossibleMoveResult {
                final_pos: *last_possible_move_loc,
                stop: true,
            };
        } else if let Some(_checker) = current_player_locations.get(&(x, y)) {
            // encountered a checker on the same team
            return FindPossibleMoveResult {
                final_pos: *last_possible_move_loc,
                stop: true,
            };
        } else {
            // this is an empty cell
            // if next cell is out of bounds, then this is definitely the possible move
            if next_cell_pos.is_none() {
                return FindPossibleMoveResult {
                    final_pos: Some((x, y)),
                    stop: true,
                };
            }

            *last_possible_move_loc = Some((x, y));
            println!("has last possible move loc: {:?}", last_possible_move_loc);
        }

        FindPossibleMoveResult {
            final_pos: None,
            stop: false,
        }
    }

    /// return some if the next cell is in any path
    fn verify_cell_pos(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let next_cell_loc = self.is_cell_in_any_path(&pos).then_some(pos);
        next_cell_loc
    }

    // pub(crate) fn get_checker(&self, pos: (usize, usize)) -> Option {

    // }
}

#[derive(Debug)]
struct PossibleMove {
    dir: Direction,
    final_pos: (usize, usize),
}

struct FindPossibleMoveResult {
    final_pos: Option<(usize, usize)>,
    stop: bool,
}
