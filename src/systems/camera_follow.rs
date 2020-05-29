use crate::resources::CameraInfo;
use crate::components::Player;
use crate::vectors::Vector2;
use amethyst::core::{
    math::{Vector3},
    Transform,
};
use amethyst::ecs::{Join, Write, ReadStorage, System, WriteStorage};
use amethyst::renderer::Camera;

const FOLLOW_SPEED: f32 = 0.05;
const MIN_MOVE_DISTANCE: f32 = 0.1;

pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        Write<'s, CameraInfo>,
    );

    fn run(&mut self, (mut transforms, _players, cameras, mut camera_info): Self::SystemData) {
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            let position = Vector2::new(transform.translation().x, transform.translation().y); 
            let target = camera_info.player_position;
            let new_position = Vector2::lerp(
                position,
                target,
                FOLLOW_SPEED,
            );
            let distance = (new_position - position).magnitude();
            if distance >= MIN_MOVE_DISTANCE {
                *transform.translation_mut() = Vector3::new(new_position.x, new_position.y, 1.0);
            }
            camera_info.camera_transform = transform.clone();
        }
    }
}
