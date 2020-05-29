use crate::components::Player;
use crate::components::Physics;
use crate::resources::CameraInfo;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::vectors::Vector2;

pub struct MovementSystem;

const MOVE_SPEED: f32 = 120.0;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Physics>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, CameraInfo>,
    );

    fn run(&mut self, (mut transforms, players, mut physics, input, mut camera_info): Self::SystemData) {
        for (_player, physic, transform) in (&players, &mut physics, &mut transforms).join() {
            let input_h = match input.axis_value("move_horizontal") {
                Some(value) => value,
                None => 0.0,
            };
            let input_v = match input.axis_value("move_vertical") {
                Some(value) => value,
                None => 0.0,
            };
            let velocity = Vector2::new(input_h, input_v).normalized() * MOVE_SPEED;
            physic.velocity = velocity;

            // TODO: Temp fix
            camera_info.player_position = Vector2::new(transform.translation().x, transform.translation().y);
        }
    }
}
