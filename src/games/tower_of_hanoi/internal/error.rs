use std::error;
use std::fmt;

#[derive(Debug)]
pub(super) struct ParseDiskMoveError {
    pub(super) message: String,
}

impl ParseDiskMoveError {
    pub(super) fn new(pole_index: usize) -> Self {
        ParseDiskMoveError {
            message: format!("Pole {} not found", pole_index),
        }
    }
}

impl fmt::Display for ParseDiskMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ParseDiskMoveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
