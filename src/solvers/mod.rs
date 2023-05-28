pub mod backtrack;

use crate::common::grid::Grid;

use self::backtrack::BacktrackFactory;

pub trait SolvingAlgorithm {
    fn solve(&mut self) -> bool;
    fn get_grid(&self) -> &Grid;
}

pub trait Factory {
    fn init(&self, grid: &Grid) -> Box<dyn SolvingAlgorithm>;
}

pub fn get_solver_factory(algorithm: &str) -> Result<&'static dyn Factory, String> {
    match algorithm {
        "backtrack" => Ok(&BacktrackFactory),
        _ => Err(format!("{} is not supported", algorithm)),
    }
}
