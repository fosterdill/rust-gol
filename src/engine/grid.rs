use super::cell::Cell;

#[derive(Debug)]
pub struct Grid (Vec<Cell>, u16);

impl Grid {
    fn build_diff_array() -> [(i8, i8); 9]  {
        [
            (0, 0),
            (0, 1),
            (0, -1),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 0),
            (-1, 1),
            (-1, -1),
        ]
    }

    fn build_grid_vector(width: u16, height: u16) -> Vec<Cell> {
        let length = width * height;
        
        (0..length)
            .map(|index| {
                let x_pos = match index {
                    0 => 0,
                    _ => index % width,
                };
                let y_pos = (index - x_pos) / width;

                Cell::new(x_pos, y_pos)
            })
            .collect::<Vec<Cell>>()
    }

    pub fn new (width: u16, height: u16) -> Grid {
        Grid (Grid::build_grid_vector(width, height), width)
    }

    pub fn borrow_cell(&self, x_pos: u16, y_pos: u16) -> &Cell {
        &self.0[(y_pos * self.1 + x_pos) as usize]
    }

    pub fn get_relatives_for(&self, x: u16, y: u16) -> Vec<&Cell> {
        Grid::build_diff_array()
            .iter()
            .map(|diff_tuple| {
                let relative_tuple: (i32, i32) = (diff_tuple.0 as i32 + x as i32, diff_tuple.1 as i32 + y as i32);

                match relative_tuple {
                    tuple if tuple.0 >= 0 && tuple.1 >= 0 && tuple.0 <= u16::max_value() as i32 && tuple.1 <= u16::max_value() as i32 => {
                        Some((tuple.0 as u16, tuple.1 as u16))
                    }
                    _ => None
                }
            })
            .filter(|option| {
                match *option {
                    Some(_) => true,
                    None => false,
                }
            })
            .map(|option| {
                let relative_tuple = option.unwrap();

                self.borrow_cell(relative_tuple.0, relative_tuple.1)
            })
            .collect::<Vec<&Cell>>()
    }
}