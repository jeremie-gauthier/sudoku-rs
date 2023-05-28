mod common;
mod sudoku;
use sudoku::{solver, Sudoku};

const HARD_GRID: [[u8; 9]; 9] = [
    [0, 0, 0, 8, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 4, 3],
    [5, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 7, 0, 8, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 2, 0, 0, 3, 0, 0, 0, 0],
    [6, 0, 0, 0, 0, 0, 0, 7, 5],
    [0, 0, 3, 4, 0, 0, 0, 0, 0],
    [0, 0, 0, 2, 0, 0, 6, 0, 0],
];

const HARD_GRID_2: [[u8; 9]; 9] = [
    [1, 0, 0, 3, 0, 0, 0, 0, 8],
    [6, 7, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 2, 0, 0, 6, 7, 0, 0, 0],
    [0, 0, 5, 0, 0, 0, 3, 0, 0],
    [0, 0, 0, 0, 4, 0, 0, 0, 0],
    [3, 0, 8, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 2, 6, 0],
    [0, 0, 0, 0, 0, 0, 0, 7, 0],
];

const MEDIUM_GRID: [[u8; 9]; 9] = [
    [0, 0, 2, 8, 0, 0, 0, 1, 0],
    [0, 7, 4, 3, 0, 1, 0, 8, 0],
    [0, 0, 0, 0, 2, 4, 0, 0, 0],
    [6, 0, 0, 5, 0, 0, 9, 0, 0],
    [0, 0, 0, 0, 8, 0, 0, 0, 0],
    [0, 0, 8, 0, 0, 2, 0, 0, 5],
    [0, 0, 0, 7, 3, 0, 0, 0, 0],
    [0, 8, 0, 4, 0, 6, 7, 2, 0],
    [0, 4, 0, 0, 0, 8, 3, 0, 0],
];

fn main() {
    let mut sudoku = Sudoku::new(HARD_GRID_2);
    println!("{}", sudoku);
    solver(&mut sudoku, 0);
    println!("{}", sudoku);
}
