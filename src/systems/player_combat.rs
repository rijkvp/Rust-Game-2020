use crate::resources::CameraInfo;
use crate::components::{Damageable, Lifetime, Physics, PhysicsLayer, PhysicsType, Player};
use crate::resources::SpriteSheetHolder;
use amethyst::core::math::{Vector3, Vector2, Point3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use crate::vectors::Vector2 as vec2;

pub struct PlayerCombat;

const FIRE_DELAY: f32 = 0.5;

impl<'s> System<'s> for PlayerCombat {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Damageable>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Physics>,
        WriteStorage<'s, Lifetime>,
        Entities<'s>,
        Read<'s, SpriteSheetHolder>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadStorage<'s, Camera>,
        Read<'s, CameraInfo>
    );

    fn run(
        &mut self,
        (
            mut players,
            mut transforms,
            mut damageables,
            mut sprite_renderers,
            mut physics,
            mut lifetimes,
            entities,
            sprite_sheet_holder,
            input_handler,
            time,
            cameras,
            camera_info
        ): Self::SystemData,
    ) {
        let sprite_sheet = match &sprite_sheet_holder.sprite_sheet {
            None => panic!("Sprite sheet not set!"),
            Some(s) => s,
        };
        let fire = input_handler.action_is_down("fire").unwrap_or(false);
        let mut player_pos = camera_info.player_position;
        let mut mouse_world_pos: vec2 = vec2::default();
        if let Some((x, y)) = input_handler.mouse_position() {
            for camera in (&cameras).join() {
                let mut trans = Transform::default();
                trans.set_translation(camera_info.player_position.to_vector3());
                let world_point = camera.projection().screen_to_world_point(
                    Point3::new(x, y, 0.0),
                    Vector2::new(1920.0, 1028.0),
                    &trans,
                );
                mouse_world_pos = vec2::new(world_point.x, world_point.y);
            }
        }
        if fire {
            for player in (&mut players).join() {
                if player.fire_timer > 0.0 {
                    player.fire_timer -= time.delta_seconds();
                }
                if player.fire_timer <= 0.0 {
                    let direction = (mouse_world_pos - player_pos).normalized();
                    player.fire_timer += FIRE_DELAY;
                    let mut projectile_transform = Transform::from(Vector3::new(player_pos.x, player_pos.y, 0.0));
                    projectile_transform.set_rotation_2d(direction.get_degrees());
                    projectile_transform.set_scale(Vector3::new(0.2, 0.2, 0.0));
                    entities
                        .build_entity()
                        .with(projectile_transform, &mut transforms)
                        .with(Damageable { damage: 50.0 }, &mut damageables)
                        .with(
                            SpriteRender {
                                sprite_sheet: sprite_sheet.clone(),
                                sprite_number: 2,
                            },
                            &mut sprite_renderers,
                        )
                        .with(
                            Physics {
                                physics_type: PhysicsType::Dynamic,
                                velocity: direction * 50.0,
                                drag: false,
                                layer: PhysicsLayer::Projectile,
                            },
                            &mut physics,
                        )
                        .with(Lifetime { lifetime: 5.0 }, &mut lifetimes)
                        .build();
                }
            }
        }
    }
}
