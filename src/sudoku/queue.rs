use crate::common::{coord::Coord, digit::Digit, grid::Grid, GRID_SIZE};

use super::{candidate::Candidates, candidate_checker::CandidateChecker};

#[derive(Debug)]
struct Item {
    coord: Coord,
    candidates: Candidates,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coord.partial_cmp(&other.coord)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coord.cmp(&other.coord)
    }
}

#[derive(Debug)]
pub struct Queue(Vec<Item>);

impl Queue {
    pub fn new(grid: &mut Grid, candidate_checker: &CandidateChecker) -> Self {
        let mut queue = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let cell = grid.get_cell_mut(row, col);
                if cell.digit.get() > 0 {
                    continue;
                }

                let mut candidates = Candidates::new();
                for n in 1..=GRID_SIZE {
                    if candidate_checker.is_a_candidate(n as u8, cell.coord) {
                        candidates.add(Digit::new(n as u8));
                    }
                }

                cell.coord.set_ord(candidates.length as u8);

                queue.push(Item {
                    coord: cell.coord,
                    candidates,
                });
            }
        }

        queue.sort_unstable();

        Self(queue)
    }

    pub fn is_exhausted(&self, idx: usize) -> bool {
        idx >= self.0.len()
    }

    pub fn get_coord(&self, idx: usize) -> Coord {
        self.0[idx].coord
    }

    pub fn get_candidates_ref(&self, idx: usize) -> &Candidates {
        &self.0[idx].candidates
    }
}
