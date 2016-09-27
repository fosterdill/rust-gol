use super::rules::{LifeState, HasLifeState};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: u16,
    y: u16,
    pub life_state: LifeState,
    pub next_life_state: LifeState,
}

impl HasLifeState for Cell {
    fn get_life_state(&self) -> &LifeState {
        &self.life_state
    }
}

impl Cell {
    pub fn new(x: u16, y: u16, life_state: LifeState) -> Cell {
        Cell { x: x, y: y, life_state: life_state, next_life_state: life_state }
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn spawn_or_stay_alive(&mut self) {
        self.next_life_state = LifeState::Alive;
    }

    pub fn kill_or_stay_dead(&mut self) {
        self.next_life_state = LifeState::Dead;
    }

    pub fn live(&mut self) {
        self.life_state = self.next_life_state;
    }
}
