// use super::grid::Grid;

#[derive(Debug)]
pub struct Cell {
    x: u16,
    y: u16,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Cell {
        Cell { x: x, y: y }
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }
}