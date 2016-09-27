use super::cell::Cell;
use super::rules::LifeState;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub cell_relatives: Vec<Vec<(u16, u16)>>,
    pub width: u16,
}

impl Grid {
    fn build_diff_array() -> [(i8, i8); 8]  {
        [
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

                if (x_pos == 1 && y_pos == 1) || (x_pos == 1 && y_pos == 2) || (x_pos == 1 && y_pos == 3) {
                    Cell::new(x_pos, y_pos, LifeState::Alive)
                } else {
                    Cell::new(x_pos, y_pos, LifeState::Dead)
                }
            })
            .collect::<Vec<Cell>>()
    }

    pub fn new (width: u16, height: u16) -> Grid {
        let cells = Grid::build_grid_vector(width, height);
        let cell_relatives = Grid::generate_relative_tuples_for(&cells, width);

        Grid {
            cells: cells,
            width: width,
            cell_relatives: cell_relatives,
        }
    }

    pub fn generate_relative_tuples_for(cells: &Vec<Cell>, width: u16) -> Vec<Vec<(u16, u16)>> {
        cells
            .iter()
            .map(|cell| {
                let x = cell.get_x();
                let y = cell.get_y();

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
                        option.unwrap()
                    })
                    .filter(|tuple| {
                        let height = (cells.len() as u16) / width;

                        tuple.0 < width && tuple.1 < height
                    })
                    .collect::<Vec<(u16, u16)>>()
            })
            .collect::<Vec<Vec<(u16, u16)>>>()
    }
}
