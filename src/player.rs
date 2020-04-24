use crate::physics::*;
use crate::event_manager::EventManager;
use crate::vectors::Vector2;

pub struct Player
{
    pub position: Vector2,
    pub collider_id: u32,
    pub is_dead: bool,
    pub health: f32,
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
            collider_id,
            is_dead: false,
            health: 100.0,
        }
    }
    
    pub fn update(&mut self, evt_manager: &EventManager, pm: &mut PhysicsManager, delta_time: f32)
    {
        let mut movement = evt_manager.get_input_vector();

        movement = movement.normalized() * 400.0 * delta_time;

        if !pm.check_collision(
        &AABB::from_center(self.position + Vector2{x: movement.x, y: 0.0}, 64.0, 64.0), &self.collider_id)
        {
            self.position += Vector2{x: movement.x, y: 0.0}
        }
        if !pm.check_collision(
        &AABB::from_center(self.position + Vector2{x: 0.0, y: movement.y}, 64.0, 64.0), &self.collider_id)
        {
            self.position += Vector2{x: 0.0, y: movement.y}
        }

        pm.update_collider(self.collider_id, AABB::from_center(self.position, 64.0, 64.0));
    }

    pub fn take_damage(&mut self, amount: f32, pm: &mut PhysicsManager)
    {
        if !self.is_dead
        {
            self.health -= amount;
            if self.health < 0.0
            {
                self.is_dead = true;
                pm.remove_collider(self.collider_id);
            }
        }
    }
}
