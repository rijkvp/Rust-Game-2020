use crate::physics::*;
use crate::vectors::Vector2;

const MOVE_SPEED: f32 = 100.0;

pub struct Enemy {
    pub position: Vector2,
    collider_id: u32,
}

impl Enemy {
    pub fn new(position: Vector2, pm: &mut PhysicsManager) -> Enemy {
        let collider = AABB::from_center(position, 64.0, 64.0);
        let collider_id = pm.add_collider(collider);
        Enemy {
            position,
            collider_id,
        }
    }

    pub fn update(&mut self, target: Vector2, pm: &mut PhysicsManager) {
        let move_distance = MOVE_SPEED * crate::DELTA_TIME;
        let move_horizontal = !pm.check_collision(
            &AABB::from_center(self.position + Vector2{x: move_distance, y: 0.0}, 64.0, 64.0),
            &self.collider_id,
        );
        let move_vertical = !pm.check_collision(
            &AABB::from_center(self.position + Vector2{x: 0.0, y: move_distance}, 64.0, 64.0),
            &self.collider_id,
        );
        let enemy_movement: Vector2 = {
            if move_horizontal || move_vertical
            {
                let mut dir = target - self.position;
                dir = dir.normalized();
                dir = dir * MOVE_SPEED * crate::DELTA_TIME;
                if move_horizontal && move_vertical {
                    dir
                } else if move_horizontal {
                    Vector2{ x: dir.x, y: 0.0 }
                } else if move_vertical {
                    Vector2{ x: 0.0, y: dir.y }
                } else {
                    Vector2::zero()
                }
            }
            else
            {
                Vector2::zero()
            }
        };
        self.position += enemy_movement;

        pm.update_collider(self.collider_id, AABB::from_center(self.position, 64.0, 64.0));
    }
}
