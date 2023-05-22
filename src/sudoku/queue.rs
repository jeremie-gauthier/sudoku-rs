use super::{candidate_checker::CandidateChecker, coord::Coord, grid::Grid, GRID_SIZE};

#[derive(Debug)]
pub struct Queue(Vec<Coord>);

impl Queue {
    pub fn new(grid: &mut Grid, candidate_checker: &CandidateChecker) -> Self {
        let mut queue = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let cell = grid.get_cell_mut(row, col);
                if cell.digit.get() > 0 {
                    continue;
                }

                let mut ord = 0;
                for n in 1..=GRID_SIZE {
                    if candidate_checker.is_a_candidate(n as u8, cell.coord) {
                        ord += 1;
                        cell.add_candidate(n as u8);
                    }
                }

                cell.coord.set_ord(ord);

                queue.push(cell.coord);
            }
        }

        queue.sort_unstable();

        Self(queue)
    }

    pub fn is_exhausted(&self, idx: usize) -> bool {
        idx >= self.0.len()
    }

    pub fn get(&self, idx: usize) -> Coord {
        self.0[idx]
    }
}
