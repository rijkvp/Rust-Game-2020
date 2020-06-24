use crate::components::{Enemy, EnemyType, Physics};
use crate::resources::GameInfo;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};

use crate::vectors::Vector2;

pub struct AISystem;

const AI_MOVE_SPEED: f32 = 40.0;
const MELEE_MIN_DIST: f32 = 16.0;
const MELEE_ATTACK_DIST: f32 = 20.0;
const RANGE_MIN_DIST: f32 = 80.0;
const MAX_VIEW_DIST: f32 = 200.0;

impl<'s> System<'s> for AISystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physics>,
        Read<'s, GameInfo>,
    );

    fn run(&mut self, (mut enemies, mut transforms, mut physics, game_info): Self::SystemData) {
        for (enemy, physic, transform) in (&mut enemies, &mut physics, &mut transforms).join() {
            let target = game_info.player_position;
            let curr_pos = Vector2::new(transform.translation().x, transform.translation().y);
            let vec = target - curr_pos;
            let distance = vec.magnitude();
            match enemy.enemy_type {
                EnemyType::Melee => {
                    if distance < MAX_VIEW_DIST {
                        if distance > MELEE_MIN_DIST {
                            let dir = vec.normalized();
                            physic.velocity = dir * AI_MOVE_SPEED;
                        } 
                        if distance < MELEE_ATTACK_DIST {
                            // MELEE ATTACK!
                            enemy.can_attack = true;
                        }
                    }
                }
                EnemyType::Range => {
                    if distance < MAX_VIEW_DIST {
                        if distance > RANGE_MIN_DIST {
                            let dir = vec.normalized();
                            physic.velocity = dir * AI_MOVE_SPEED;
                            // RANGE ATTACK!
                            enemy.can_attack = true;
                        }
                    }
                }
            }
        }
    }
}
