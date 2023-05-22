mod candidate;
mod candidate_checker;
mod cell;
mod checker;
mod coord;
mod digit;
mod grid;
mod queue;
mod solver;

use std::fmt;

pub use checker::checker;
pub use solver::solver;

use self::{candidate_checker::CandidateChecker, grid::Grid, queue::Queue};

pub const GRID_SIZE: usize = 9;

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
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.grid)
    }
}
