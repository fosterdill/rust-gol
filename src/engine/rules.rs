#[derive(Debug)]
pub struct Rules;

#[derive(Debug, Clone, Copy)]
pub enum LifeState {
    Alive,
    Dead,
}

pub trait HasLifeState {
    fn get_life_state(&self) -> &LifeState;
}

pub trait ConwayRules<T> {
    fn get_next_life_state(&self, &T, Vec<T>) -> LifeState;
}

impl <T: HasLifeState> ConwayRules<T> for Rules {
    fn get_next_life_state(&self, cell: &T, relatives: Vec<T>) -> LifeState {
        let living_relatives_count = relatives
            .iter()
            .filter(|cell| {
                match cell.get_life_state() {
                    &LifeState::Alive => true,
                    _ => false,
                }
            })
            .collect::<Vec<&T>>()
            .len();

        match cell.get_life_state() {
            &LifeState::Alive => {
                match living_relatives_count {
                    2 | 3 => LifeState::Alive,
                    _ => LifeState::Dead,
                }
            }
            &LifeState::Dead => {
                match living_relatives_count {
                    3 => LifeState::Alive,
                    _ => LifeState::Dead,
                }
            }
        }
    }
}
