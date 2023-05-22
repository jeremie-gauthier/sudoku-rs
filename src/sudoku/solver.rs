use super::Sudoku;

pub fn solver(sudoku: &mut Sudoku, idx: usize) -> bool {
    if sudoku.queue.is_exhausted(idx) {
        return true;
    }

    let coord = sudoku.queue.get(idx).clone();
    if sudoku.grid.get(coord).digit.get() > 0 {
        return solver(sudoku, idx + 1);
    }

    let candidates = &sudoku.grid.get(coord).candidates;

    for candidate in candidates.data {
        let value = candidate.get();
        if value == 0 {
            break;
        }

        if !sudoku.candidate_checker.can_set(candidate, coord) {
            continue;
        }
        sudoku.grid.get_mut(coord).digit.set(value);
        sudoku.candidate_checker.set(value, coord);

        if solver(sudoku, idx + 1) {
            return true;
        }

        sudoku.grid.get_mut(coord).digit.set(0);
        sudoku.candidate_checker.unset(value, coord);
    }

    false
}
