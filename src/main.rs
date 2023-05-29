mod common;
mod solvers;

use common::grid::Grid;
use solvers::get_solver_factory;
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let grid = Grid::from(buffer);

    let factory = get_solver_factory("backtrack").unwrap();
    let mut solver = factory.init(&grid);
    solver.solve();
    println!("{}\n\n{}", grid, solver.get_grid());
}
