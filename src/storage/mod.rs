pub mod bitfield;

use crate::common::{coord::Coord, digit::Digit, grid::Grid};
pub use bitfield::BitFieldAdapter;

pub trait Storage {
    fn load_grid(&mut self, grid: &Grid);
    fn has_digit_at_coord(&self, digit: Digit, coord: Coord) -> bool;
    fn set_digit_at_coord(&mut self, digit: Digit, coord: Coord);
    fn remove_digit_at_coord(&mut self, digit: Digit, coord: Coord);
}
