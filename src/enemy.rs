use crate::physics::*;
use crate::vectors::Vector2;

pub struct Enemy
{
    pub position: Vector2,
    collider_id: u32,
}

impl Enemy
{
    pub fn new(position: Vector2, pm: &mut PhysicsManager) -> Enemy
    {
        let collider = AABB::from_center(position, 64.0, 64.0);
        let collider_id = pm.add_collider(collider);
        Enemy {
            position,
            collider_id
        }
    }

    pub fn update(&mut self, target: Vector2, pm: &mut PhysicsManager)
    {
        let collider = AABB::from_center(self.position, 64.0, 64.0);
        let mut dir = target - self.position;
        if dir.magnitude() > 64.0
        {
            dir = dir.normalized();
            let enemy_movement = dir * 100.0 * crate::DELTA_TIME;
            if !pm.check_collision(&AABB::from_center(self.position + enemy_movement, 64.0, 64.0), &self.collider_id)
            {
                self.position = self.position + enemy_movement
            }
        }
        pm.update_collider(self.collider_id, collider);
    }
}
