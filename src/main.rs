extern crate gol;

use gol::engine::*;

fn main () {
    let grid = grid::Grid::new(5, 5);
    let rules = rules::Rules;
    let mut lifecycle = lifecycle::Lifecycle {
        rules: rules,
        grid: grid,
    };

    lifecycle.display();
    lifecycle.iterate();
    lifecycle.display();
}
