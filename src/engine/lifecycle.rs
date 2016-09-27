use super::grid::Grid;
use super::cell::Cell;
use super::rules::*;

pub struct Lifecycle {
    pub rules: Rules,
    pub grid: Grid,
}

impl Lifecycle {
    pub fn display(&self) {
        let height = self.grid.cells.len() as u16 / self.grid.width;

        for y in (0..height) {
            print!("\n");
            for x in (0..self.grid.width) {
                let graphic = match self.grid.cells[(y * height + x) as usize].get_life_state() {
                    &LifeState::Alive => 'X',
                    &LifeState::Dead => '0',
                };

                print!("{:?}", graphic);
            }
            print!("\n");
        }
    }

    pub fn iterate(&mut self) {
        for cell_index in (0..self.grid.cells.len()) {
            let relative_cells = self.grid.cell_relatives[cell_index]
                .iter()
                .map(|relative_cell_position| {
                    self.grid.cells[(relative_cell_position.1 * (self.grid.cells.len() as u16 / self.grid.width) + relative_cell_position.0) as usize]
                })
                .collect::<Vec<Cell>>();
            let cell = &mut self.grid.cells[cell_index];

            match self.rules.get_next_life_state(cell, relative_cells) {
                LifeState::Dead => cell.kill_or_stay_dead(),
                LifeState::Alive => cell.spawn_or_stay_alive(),
            }
        }
        for cell in self.grid.cells.iter_mut() {
            cell.live();
        }
    }
}
