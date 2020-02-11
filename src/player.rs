use crate::physics::*;
use crate::event_manager::EventManager;
use crate::vectors::Vector2;

pub struct Player
{
    pub position: Vector2,
    collider_id: u32,
}

impl Player
{
    pub fn new(position: Vector2, pm: &mut PhysicsManager) -> Player
    {
        let collider = AABB::from_center(position, 64.0, 64.0);
        let collider_id = pm.add_collider(collider);
        Player
        {
            position,
            collider_id
        }
    }
    pub fn update(&mut self, evt_manager: &EventManager, pm: &mut PhysicsManager)
    {
        let mut movement = evt_manager.get_input_vector();

        movement = movement.normalized() * 200.0 * crate::DELTA_TIME;
        self.position = self.position + movement; // TODO: Implement AddAssign for += and -= SubAssign

        pm.update_collider(self.collider_id, AABB::from_center(self.position, 64.0, 64.0));
    }
}
