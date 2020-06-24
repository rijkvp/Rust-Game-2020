use crate::components::{Damageable, Lifetime, Physics, PhysicsLayer, PhysicsType, Player};
use crate::resources::GameInfo;
use crate::resources::SpriteSheetHolder;
use crate::resources::{play_fire_sound, Sounds};
use crate::vectors::Vector2 as vec2;
use amethyst::core::math::{Point3, Vector2, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read,  ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
};
use std::ops::Deref;

pub struct PlayerCombatSystem;

const FIRE_DELAY: f32 = 0.2;
const PROJECTILE_SPEED: f32 = 200.0;
const PROJECTILE_SPAWN_OFFSET: f32 = 22.0;
const PROJECTILE_DAMAGE: f32 = 40.0;

impl<'s> System<'s> for PlayerCombatSystem {
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
        Read<'s, GameInfo>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            camera_info,
            asset_storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        let sprite_sheet = match &sprite_sheet_holder.sprite_sheet {
            None => return,
            Some(s) => s,
        };
        let fire = input_handler.action_is_down("fire").unwrap_or(false);
        let player_pos = camera_info.player_position;
        let cam_pos = vec2::new(
            camera_info.camera_transform.translation().x,
            camera_info.camera_transform.translation().y,
        );
        let mut mouse_world_pos: vec2 = vec2::default();
        if let Some((x, y)) = input_handler.mouse_position() {
            for camera in (&cameras).join() {
                let world_point = camera.projection().screen_to_world_point(
                    Point3::new(x, y, 0.0),
                    Vector2::new(1920.0, 1080.0), // TODO TEMP FIX
                    &camera_info.camera_transform,
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
                    let direction = (mouse_world_pos - cam_pos).normalized();
                    player.fire_timer += FIRE_DELAY;
                    let spawn_position = player_pos + direction * PROJECTILE_SPAWN_OFFSET;
                    let mut projectile_transform =
                        Transform::from(Vector3::new(spawn_position.x, spawn_position.y, 0.0));

                    projectile_transform.set_rotation_2d(-direction.get_radians());
                    projectile_transform.set_scale(Vector3::new(0.3, 0.3, 1.0));
                    entities
                        .build_entity()
                        .with(projectile_transform, &mut transforms)
                        .with(
                            Damageable {
                                damage: PROJECTILE_DAMAGE,
                                destroyed: false,
                            },
                            &mut damageables,
                        )
                        .with(
                            SpriteRender {
                                sprite_sheet: sprite_sheet.clone(),
                                sprite_number: 2,
                            },
                            &mut sprite_renderers,
                        )
                        .with(
                            Physics::simple(
                                PhysicsType::Dynamic,
                                PhysicsLayer::Projectile,
                                direction * PROJECTILE_SPEED,
                            ),
                            &mut physics,
                        )
                        .with(Lifetime { lifetime: 5.0 }, &mut lifetimes)
                        .build();
                    play_fire_sound(
                        &*sounds,
                        &asset_storage,
                        audio_output.as_ref().map(|o| o.deref()),
                    );
                }
            }
        }
    }
}
