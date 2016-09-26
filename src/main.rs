extern crate gol;

use gol::engine::*;

fn main () {
    let grid = grid::Grid::new(5, 5);
    println!("{:?}", grid);
}