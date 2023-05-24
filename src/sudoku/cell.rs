use super::{candidate::Candidates, coord::Coord, digit::Digit};
use std::fmt;

#[derive(Debug)]
pub struct Cell {
    pub digit: Digit,
    pub coord: Coord,
    pub candidates: Candidates,
}

impl Cell {
    pub fn new(digit: u8, row_idx: usize, col_idx: usize) -> Self {
        Self {
            digit: Digit::new(digit),
            coord: Coord::new(row_idx, col_idx),
            candidates: Candidates::new(),
        }
    }

    pub fn add_candidate(&mut self, candidate: u8) {
        self.candidates.add(Digit::new(candidate))
    }

    pub fn is_free(&self) -> bool {
        self.digit.get() == 0
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.digit)
    }
}
