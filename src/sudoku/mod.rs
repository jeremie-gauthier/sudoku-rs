mod candidate;
mod candidate_checker;
mod queue;
mod solver;

use std::fmt;

pub use solver::solver;

use crate::common::{coord::Coord, digit::Digit, grid::Grid, GRID_SIZE};

use self::{candidate_checker::CandidateChecker, queue::Queue};

pub struct Sudoku {
    grid: Grid,
    queue: Queue,
    candidate_checker: CandidateChecker,
}

impl Sudoku {
    pub fn new(data: [[u8; GRID_SIZE]; GRID_SIZE]) -> Self {
        let mut grid = Grid::new(data);
        let candidate_checker = CandidateChecker::new(&grid);
        let queue = Queue::new(&mut grid, &candidate_checker);

        Self {
            grid,
            queue,
            candidate_checker,
        }
    }

    #[inline]
    pub fn set_digit_at(&mut self, digit: Digit, coord: Coord) {
        let value = digit.get();
        self.grid.get_mut_ref(coord).digit.set(value);
        self.candidate_checker.set(value, coord);
    }

    #[inline]
    pub fn unset_digit_at(&mut self, coord: Coord) {
        let cell = self.grid.get_mut_ref(coord);
        let value = cell.digit.get();

        cell.digit.clear();
        self.candidate_checker.unset(value, coord);
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.grid)
    }
}
