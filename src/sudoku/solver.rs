use super::Sudoku;

pub fn solver(sudoku: &mut Sudoku, idx: usize) -> bool {
    if sudoku.queue.is_exhausted(idx) {
        return true;
    }

    let coord = sudoku.queue.get_coord(idx);
    if !sudoku.grid.get_cell_ref(coord).is_free() {
        return solver(sudoku, idx + 1);
    }

    let candidates = sudoku.queue.get_candidates_ref(idx);
    candidates.into_iter().any(|candidate| {
        if !sudoku.candidate_checker.can_set(candidate, coord) {
            return false;
        }

        sudoku.set_digit_at(candidate, coord);

        if solver(sudoku, idx + 1) {
            return true;
        }

        sudoku.unset_digit_at(coord);

        return false;
    })
}
