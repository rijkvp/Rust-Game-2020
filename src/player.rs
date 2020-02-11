use crate::event_manager::EventManager;
use crate::vectors::Vector2;

pub struct Player
{
    pub position: Vector2
}

impl Player
{
    pub fn update(&mut self, evt_manager: &EventManager)
    {
        let mut movement = evt_manager.get_input_vector();

        movement = movement.normalized() * 200.0 * crate::DELTA_TIME;
        self.position = self.position + movement; // TODO: Implement AddAssign for += and -= SubAssign
    }
}
