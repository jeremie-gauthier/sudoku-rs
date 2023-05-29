mod candidate;
mod candidate_checker;
mod queue;

use self::{candidate_checker::CandidateChecker, queue::Queue};
use super::{Factory, SolvingAlgorithm};
use crate::common::{coord::Coord, digit::Digit, grid::Grid};
use std::fmt;

pub struct BacktrackFactory;
impl Factory for BacktrackFactory {
    fn init(&self, grid: &Grid) -> Box<dyn SolvingAlgorithm> {
        let mut grid = grid.clone();
        let candidate_checker = CandidateChecker::new(&grid);
        let queue = Queue::new(&mut grid, &candidate_checker);

        Box::new(Backtrack {
            grid,
            queue,
            candidate_checker,
        })
    }
}

pub struct Backtrack {
    grid: Grid,
    queue: Queue,
    candidate_checker: CandidateChecker,
}

impl Backtrack {
    fn run(&mut self, idx: usize) -> bool {
        if self.queue.is_exhausted(idx) {
            return true;
        }

        let coord = self.queue.get_coord(idx);
        if !self.grid.get_cell_ref(coord).is_free() {
            return self.run(idx + 1);
        }

        let candidates = self.queue.get_candidates_ref(idx);
        candidates.into_iter().any(|candidate| {
            if !self.candidate_checker.can_set(candidate, coord) {
                return false;
            }

            self.set_digit_at(candidate, coord);

            if self.run(idx + 1) {
                return true;
            }

            self.unset_digit_at(coord);

            false
        })
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

impl SolvingAlgorithm for Backtrack {
    fn solve(&mut self) -> bool {
        self.run(0)
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }
}

impl fmt::Display for Backtrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.grid)
    }
}
