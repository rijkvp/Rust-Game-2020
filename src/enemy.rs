use crate::bullet::{Bullet, BulletsManager};
use crate::player::Player;
use crate::physics::*;
use crate::vectors::Vector2;

const MOVE_SPEED: f32 = 300.0;
const MAX_MOVE_DISTANCE: f32 = 800.0;
const MIN_MOVE_DISTANCE: f32 = 200.0;
const MELEE_ATTACK_RANGE: f32 = 80.0;
const RANGE_ATTACK_RANGE: f32 = 500.0;
const MELEE_DPS: f32 = 30.0;
const FIRE_DELAY: f32 = 0.1;

#[derive(Copy, Clone)]
pub enum EnemyType
{
    Melee, 
    Range,
}

pub struct Enemy {
    pub position: Vector2,
    pub collider_id: u32,
    pub is_dead: bool,
    pub enemy_type: EnemyType,
    health: f32,
    fire_timer: f32,
}

impl Enemy {
    pub fn new(position: Vector2, pm: &mut PhysicsManager, enemy_type: EnemyType) -> Enemy {
        let collider = AABB::from_center(position, 64.0, 64.0);
        let collider_id = pm.add_collider(collider);
        Enemy {
            position,
            collider_id,
            health: 100.0,
            is_dead: false,
            enemy_type,
            fire_timer: 0.0,
        }
    }

    pub fn update(&mut self, target: &mut Player, pm: &mut PhysicsManager, bullets_manager: &mut BulletsManager, delta_time: f32) {
        
        let distance_to_target = Vector2::distance(self.position, target.position);

        // Movement
        let can_move = match self.enemy_type
        {
            EnemyType::Melee => distance_to_target < MAX_MOVE_DISTANCE, 
            EnemyType::Range => distance_to_target > MIN_MOVE_DISTANCE && distance_to_target < MAX_MOVE_DISTANCE,
        };
        if can_move
        {
            let move_distance = MOVE_SPEED * delta_time;
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
                    let mut dir = target.position - self.position;
                    dir = dir.normalized();
                    dir = dir * MOVE_SPEED * delta_time;
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

        // Combat
        match self.enemy_type
        {
            EnemyType::Melee => {
                if distance_to_target <= MELEE_ATTACK_RANGE
                {
                    self.melee_attack(target, pm, delta_time);
                }
            },
            EnemyType::Range => {
                if distance_to_target <= RANGE_ATTACK_RANGE
                {
                    self.fire_timer -= delta_time;
                    if self.fire_timer <= 0.0
                    {
                        let dir = (target.position - self.position).normalized();
                        let position_offset = dir * 60.0;
                        bullets_manager.add_bullet(Bullet::new(self.position + position_offset, dir));
                        self.fire_timer = FIRE_DELAY;
                    }
                }
            }
        }
    }

    fn melee_attack(&self, player: &mut Player, pm: &mut PhysicsManager, delta_time: f32)
    {
        player.take_damage(delta_time * MELEE_DPS, pm)
    }
    
    pub fn take_damage(&mut self, amount: f32, pm: &mut PhysicsManager)
    {
        if !self.is_dead
        {
            self.health -= amount;
            if self.health < 0.0
            {
                self.health = 0.0;
                self.is_dead = true;
                pm.remove_collider(self.collider_id);
            }
        }
    }
}
