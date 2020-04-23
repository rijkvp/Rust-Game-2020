use crate::enemy::Enemy;
use crate::physics::*;
use crate::vectors::Vector2;

const MAX_LIFETIME: f32 = 20.0;
const BULLET_SPEED: f32 = 700.0;


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

    pub fn update(&mut self, pm: &mut PhysicsManager, enemies: &mut Vec<Enemy>, delta_time: f32) {
        if self.is_destroyed
        {
            return;
        }

        self.position += self.direction * BULLET_SPEED * delta_time;
        self.lifetime += delta_time;
        if self.lifetime >= MAX_LIFETIME
        {
            self.is_destroyed = true;
        }
        let id = pm.check_collision_id(&AABB::from_center(self.position, 10.0, 10.0), &0);
        if id != 0
        {
            for enemy in enemies.iter_mut() {
                if enemy.collider_id == id
                {
                    enemy.take_damage(20.0, pm);
                }
            }
            self.is_destroyed = true;
        }
    }

    // Get the rotation in degrees
    pub fn get_rotation(&self) -> f32
    {
        self.direction.get_degrees()
    }
}