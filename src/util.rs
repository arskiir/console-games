use std::io::stdin;
use std::time::{Duration, Instant};

use random_number::random;

pub fn get_char_input() -> Option<char> {
    let mut input = String::new();
    while input.is_empty() {
        stdin().read_line(&mut input).expect("Failed to read input");
    }
    input.trim().chars().next()
}

pub struct TimeCounter {
    start_time: Option<Instant>,
    stop_time: Option<Instant>,
}

impl TimeCounter {
    pub fn new() -> TimeCounter {
        TimeCounter {
            start_time: None,
            stop_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.stop_time = Some(Instant::now());
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.start_time, self.stop_time) {
            (Some(start), Some(stop)) => Some(stop - start),
            _ => None,
        }
    }
}

pub fn probability(percent: u8) -> bool {
    assert!(percent <= 100);
    random!(..=100) < percent
}
