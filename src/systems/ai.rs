use crate::components::{Physics, Enemy};
use crate::resources::GameInfo;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::vectors::Vector2;

pub struct AISystem;

const AI_MOVE_SPEED: f32 = 40.0;
const MELEE_MIN_DIST: f32 = 50.0;
const MAX_VIEW_DIST: f32 = 100.0;

impl<'s> System<'s> for AISystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physics>,
        Read<'s, GameInfo>,
    );

    fn run(&mut self, (enemies, mut transforms, mut physics, game_info): Self::SystemData) {
        for (_enemy, physic, transform) in (&enemies, &mut physics, &mut transforms).join() {
            let target = game_info.player_position;
            let curr_pos = Vector2::new(transform.translation().x, transform.translation().y);
            let vec = target - curr_pos;
            if vec.magnitude() < MAX_VIEW_DIST
            {
                let dir = vec.normalized();
                physic.velocity = dir * AI_MOVE_SPEED;
            }
        }
    }
}
