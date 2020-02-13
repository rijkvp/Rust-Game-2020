use crate::physics::*;
use crate::vectors::Vector2;

const MAX_LIFETIME: f32 = 5.0;

pub struct Bullet {
    pub position: Vector2,
    pub direction: Vector2,
    lifetime: f32,
    pub is_destroyed: bool
}

impl Bullet {
    pub fn new(position: Vector2, direction: Vector2) -> Bullet {
        Bullet {
            position,
            direction,
            lifetime: 0.0,
            is_destroyed: false
        }
    }

    pub fn update(&mut self, pm: &mut PhysicsManager, ) {
        if self.is_destroyed
        {
            return;
        }

        self.position += self.direction * 200.0 * crate::DELTA_TIME;
        self.lifetime += crate::DELTA_TIME;
        if self.lifetime >= MAX_LIFETIME
        {
            self.is_destroyed = true;
        }
        let id = pm.check_collision_id(&AABB::from_center(self.position, 10.0, 10.0), &0);
        if id != 0
        {
            self.is_destroyed = true;
        }
    }
}