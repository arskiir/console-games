pub struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    adjacent_count: usize,
}

impl Cell {
    pub fn new(is_mine: bool) -> Self {
        Self {
            is_mine,
            is_revealed: false,
            adjacent_count: 0,
            is_flagged: false,
        }
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub fn adjacent_count(&self) -> usize {
        self.adjacent_count
    }

    pub fn set_adjacent_count(&mut self, adjacent_count: usize) {
        self.adjacent_count = adjacent_count;
    }

    pub fn is_flagged_mut(&mut self) -> &mut bool {
        &mut self.is_flagged
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }
}
