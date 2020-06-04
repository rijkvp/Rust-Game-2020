use crate::components::{Player, Physics, Enemy};
use crate::resources::GameInfo;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::vectors::Vector2;

pub struct AISystem;

const AI_MOVE_SPEED: f32 = 120.0;

impl<'s> System<'s> for AISystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physics>,
        Read<'s, GameInfo>,
    );

    fn run(&mut self, (enemies, mut transforms, mut physics, game_info): Self::SystemData) {
        for (_enemy, physic, transform) in (&enemies, &mut physics, &mut transforms).join() {
            // TODO: Calculate direction to enemy & set velocity
        }
    }
}
