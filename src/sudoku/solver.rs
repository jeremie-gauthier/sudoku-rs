use super::Sudoku;

pub fn solver(sudoku: &mut Sudoku, idx: usize) -> bool {
    if sudoku.queue.is_exhausted(idx) {
        return true;
    }

    let coord = sudoku.queue.get(idx).clone();
    if !sudoku.grid.get_cell_ref(coord).is_free() {
        return solver(sudoku, idx + 1);
    }

    let candidates = sudoku.grid.get_cell_ref(coord).get_candidates_ref();
    candidates.into_iter().any(|candidate| {
        if !sudoku.candidate_checker.can_set(candidate, coord) {
            return false;
        }

        let value = candidate.get();
        sudoku.grid.get_mut(coord).digit.set(value);
        sudoku.candidate_checker.set(value, coord);

        if solver(sudoku, idx + 1) {
            return true;
        }

        sudoku.grid.get_mut(coord).digit.set(0);
        sudoku.candidate_checker.unset(value, coord);
        return false;
    })
}
