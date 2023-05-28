use crate::{
    common::{coord::Coord, digit::Digit, grid::Grid},
    storage::Storage,
};

pub struct CandidateChecker(Box<dyn Storage>);

impl CandidateChecker {
    pub fn new(grid: &Grid, mut storage: Box<dyn Storage>) -> Self {
        storage.load_grid(grid);
        Self(storage)
    }

    pub fn can_set(&self, candidate: Digit, coord: Coord) -> bool {
        !self.0.has_digit_at_coord(candidate, coord)
    }

    pub fn set(&mut self, digit: Digit, coord: Coord) {
        self.0.set_digit_at_coord(digit, coord);
    }

    pub fn remove(&mut self, digit: Digit, coord: Coord) {
        self.0.remove_digit_at_coord(digit, coord);
    }
}
