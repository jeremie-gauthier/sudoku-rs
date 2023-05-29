use crate::common::{cell::Cell, coord::Coord, digit::Digit, grid::Grid, GRID_SIZE};

type BitField = u16;

pub struct CandidateChecker {
    rows: [BitField; GRID_SIZE],
    cols: [BitField; GRID_SIZE],
    squares: [BitField; GRID_SIZE],
}

impl CandidateChecker {
    pub fn new(grid: &Grid) -> Self {
        let mut rows: [BitField; GRID_SIZE] = [0; GRID_SIZE];
        let mut cols: [BitField; GRID_SIZE] = [0; GRID_SIZE];
        let mut squares: [BitField; GRID_SIZE] = [0; GRID_SIZE];

        for (row_idx, row) in rows.iter_mut().enumerate() {
            *row = parse_array(grid.get_row(row_idx));

            for (col_idx, col) in cols.iter_mut().enumerate() {
                let cell = grid.get_cell(row_idx, col_idx);
                let cell_value = cell.digit.get();
                if cell_value == 0 {
                    continue;
                }

                *col = set_bit(*col, cell_value);
                squares[cell.coord.square] = set_bit(squares[cell.coord.square], cell_value);
            }
        }

        Self {
            rows,
            cols,
            squares,
        }
    }

    pub fn is_a_candidate(&self, value: u8, coord: Coord) -> bool {
        !is_bit_set(self.rows[coord.row], value)
            && !is_bit_set(self.cols[coord.col], value)
            && !is_bit_set(self.squares[coord.square], value)
    }

    pub fn can_set(&self, candidate: Digit, coord: Coord) -> bool {
        self.is_a_candidate(candidate.get(), coord)
    }

    pub fn set(&mut self, value: u8, coord: Coord) {
        self.rows[coord.row] = set_bit(self.rows[coord.row], value);
        self.cols[coord.col] = set_bit(self.cols[coord.col], value);
        self.squares[coord.square] = set_bit(self.squares[coord.square], value);
    }

    pub fn unset(&mut self, value: u8, coord: Coord) {
        self.rows[coord.row] = unset_bit(self.rows[coord.row], value);
        self.cols[coord.col] = unset_bit(self.cols[coord.col], value);
        self.squares[coord.square] = unset_bit(self.squares[coord.square], value);
    }
}

fn set_bit(origin: BitField, value: u8) -> BitField {
    origin | (1 << (value - 1))
}

fn unset_bit(origin: BitField, value: u8) -> BitField {
    origin ^ (1 << (value - 1))
}

fn is_bit_set(origin: BitField, value: u8) -> bool {
    origin & (1 << (value - 1)) != 0
}

fn parse_array(array: &Vec<Cell>) -> BitField {
    let mut bit_field: BitField = 0;
    for cell in array {
        let value = cell.digit.get();
        if value == 0 {
            continue;
        }

        bit_field = set_bit(bit_field, value)
    }
    bit_field
}
