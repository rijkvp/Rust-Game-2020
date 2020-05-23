use crate::game::Player;
use amethyst::core::{SystemDesc, Transform, math::Vector3};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::Camera;

pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (mut transforms, players, cameras): Self::SystemData) {
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            transform.move_right(0.1);
        }
        // let mut target:  &Vector3<f32> = &Vector3::new(0.0, 0.0, 0.0);
        // for (_player, transform) in (&players, &transforms).join() {
        //     target = transform.translation();
        // }
        // for (_camera, transform) in (&cameras, &mut transforms).join() {
        //     transform.set_translation_xyz(target.x, target.y, 1.0);
        // }
    }
}
