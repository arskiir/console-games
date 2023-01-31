pub struct Cell {
    is_mine: bool,
    is_revealed: bool,
}

impl Cell {
    pub fn new(is_mine: bool) -> Self {
        Self {
            is_mine,
            is_revealed: false,
        }
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }
}
