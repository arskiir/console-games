pub struct Cell {
    is_mine: bool,
    is_revealed: bool,
    adjacent_count: u8,
}

impl Cell {
    pub fn new(is_mine: bool) -> Self {
        Self {
            is_mine,
            is_revealed: false,
            adjacent_count: 0,
        }
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub fn adjacent_count(&self) -> u8 {
        self.adjacent_count
    }

    pub fn set_adjacent_count(&mut self, adjacent_count: u8) {
        self.adjacent_count = adjacent_count;
    }
}
