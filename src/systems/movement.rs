use amethyst::core::{SystemDesc, Transform, math::Vector3};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::game::{Player, ARENA_HEIGHT};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (_player, transform) in (&players, &mut transforms).join() {
            let mut movement_h = match input.axis_value("move_horizontal") {
                Some(value) => value,
                None => 0.0,
            };
            let mut movement_v = match input.axis_value("move_vertical") {
                Some(value) => value,
                None => 0.0,
            };
            let magnitude = (movement_h * movement_h + movement_v * movement_v).sqrt();
            if magnitude > 0.0
            {
                movement_h /= magnitude;
                movement_v /= magnitude;    
            }
            transform.append_translation_xyz(movement_h, movement_v, 0.0);
        }
    }
}
