use super::{cell::Cell, coord::Coord, GRID_SIZE};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Grid(Vec<Vec<Cell>>);

impl Grid {
    pub fn get_cell_ref(&self, coord: Coord) -> &Cell {
        &self.0[coord.row][coord.col]
    }

    pub fn get_mut_ref(&mut self, coord: Coord) -> &mut Cell {
        &mut self.0[coord.row][coord.col]
    }

    pub fn get_row(&self, row_idx: usize) -> &Vec<Cell> {
        &self.0[row_idx]
    }

    pub fn get_cell(&self, row_idx: usize, col_idx: usize) -> &Cell {
        &self.0[row_idx][col_idx]
    }

    pub fn get_cell_mut(&mut self, row_idx: usize, col_idx: usize) -> &mut Cell {
        &mut self.0[row_idx][col_idx]
    }
}

impl From<String> for Grid {
    fn from(input: String) -> Self {
        let grid = input
            .chars()
            .collect::<Vec<char>>()
            .chunks(GRID_SIZE)
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, char_nb)| {
                        let digit: u8 = match *char_nb {
                            '.' => 0,
                            _ => char_nb.to_string().parse::<u8>().unwrap(),
                        };
                        Cell::new(digit, row_idx, col_idx)
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();

        Self(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_stringified = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", data_stringified)
    }
}
