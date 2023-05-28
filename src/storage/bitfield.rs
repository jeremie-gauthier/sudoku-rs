use crate::common::{cell::Cell, coord::Coord, digit::Digit, GRID_SIZE};

use super::Storage;

type BitField = u16;

pub struct BitFieldAdapter {
    rows: [BitField; GRID_SIZE],
    cols: [BitField; GRID_SIZE],
    squares: [BitField; GRID_SIZE],
}

impl BitFieldAdapter {
    pub fn new() -> Self {
        Self {
            rows: [0; GRID_SIZE],
            cols: [0; GRID_SIZE],
            squares: [0; GRID_SIZE],
        }
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

impl Storage for BitFieldAdapter {
    fn load_grid(&mut self, grid: &crate::common::grid::Grid) {
        let add_to_bitfield = |bitfield: BitField, cell: &Cell| {
            let value = cell.digit.get();
            if value == 0 {
                return bitfield;
            }

            set_bit(bitfield, value)
        };

        for row in 0..GRID_SIZE {
            self.rows[row] = grid.get_row(row).iter().fold(0, add_to_bitfield);

            for col in 0..GRID_SIZE {
                let cell = grid.get_cell(row, col);

                self.cols[col] = add_to_bitfield(self.cols[col], cell);
                self.squares[cell.coord.square] =
                    add_to_bitfield(self.squares[cell.coord.square], cell);
            }
        }
    }

    fn has_digit_at_coord(&self, digit: Digit, coord: Coord) -> bool {
        is_bit_set(self.rows[coord.row], digit.get())
            || is_bit_set(self.cols[coord.col], digit.get())
            || is_bit_set(self.squares[coord.square], digit.get())
    }

    fn set_digit_at_coord(&mut self, digit: Digit, coord: Coord) {
        self.rows[coord.row] = set_bit(self.rows[coord.row], digit.get());
        self.cols[coord.col] = set_bit(self.cols[coord.col], digit.get());
        self.squares[coord.square] = set_bit(self.squares[coord.square], digit.get());
    }

    fn remove_digit_at_coord(&mut self, digit: Digit, coord: Coord) {
        self.rows[coord.row] = unset_bit(self.rows[coord.row], digit.get());
        self.cols[coord.col] = unset_bit(self.cols[coord.col], digit.get());
        self.squares[coord.square] = unset_bit(self.squares[coord.square], digit.get());
    }
}
