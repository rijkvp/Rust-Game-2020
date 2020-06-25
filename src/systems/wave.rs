use crate::game::ARENA_WIDTH;
use crate::game::ARENA_HEIGHT;
use crate::components::{Enemy, EnemyType, Health, Physics, PhysicsType, PhysicsLayer};
use crate::resources::{GameInfo, SpriteSheetHolder};
use crate::vectors::Vector2;

use amethyst::core::math::{Point3, Vector3};
use amethyst::ecs::{Entities, Join, Read, System, Write, WriteStorage};
use amethyst::core::Transform;
use amethyst::renderer::SpriteRender;

use rand::Rng;

pub struct WaveSystem;
pub const START_ENEMY_COUNT: u16 = 8;
pub const ENEMY_WAVE_MULTIPLIER: f32 = 2.0;

impl<'s> System<'s> for WaveSystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        Entities<'s>,
        Write<'s, GameInfo>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, SpriteSheetHolder>,
        WriteStorage<'s, Physics>,
        WriteStorage<'s, Health>,
    );

    fn run(
        &mut self,
        (
            mut enemies,
            entities,
            mut game_info,
            mut transforms,
            mut sprite_renderers,
            sprite_sheet_holder,
            mut physics,
            mut healths,
        ): Self::SystemData,
    ) {
        let mut enemy_count = 0u16;
        for _enemy in enemies.join() {
            enemy_count += 1;
        }
        if enemy_count == 0 && game_info.in_game {
            game_info.wave += 1;
            // Spawn new wave
            let enemy_count =
                START_ENEMY_COUNT + (ENEMY_WAVE_MULTIPLIER * game_info.wave as f32).ceil() as u16;
            println!(
                "SPAWNING WAVE {} WITH {} ENEMIES!",
                game_info.wave, enemy_count
            );
            let sprite_sheet = match &sprite_sheet_holder.sprite_sheet {
                None => return,
                Some(s) => s,
            };
            let mut rng = rand::thread_rng();
            for i in 0..enemy_count {
                let mut enemy_transform = Transform::default();
                enemy_transform.set_scale(Vector3::new(0.2, 0.2, 0.0));
                enemy_transform.set_translation_xyz(
                    rng.gen_range(-ARENA_WIDTH / 8.0, ARENA_WIDTH / 8.0),
                    rng.gen_range(-ARENA_HEIGHT / 8.0, ARENA_HEIGHT / 8.0),
                    0.0,
                );
                let enemy = Enemy::random();
                let sprite_number = match enemy.enemy_type {
                    EnemyType::Melee => 1,
                    EnemyType::Range => 4,
                };
                entities
                    .build_entity()
                    .with(enemy, &mut enemies)
                    .with(enemy_transform, &mut transforms)
                    .with(SpriteRender {
                        sprite_sheet: sprite_sheet.clone(),
                        sprite_number,
                    }, &mut sprite_renderers)
                    .with(Physics::with_id(
                        PhysicsType::Dynamic,
                        PhysicsLayer::None,
                        Vector2::default(),
                        true,
                        2 + i,
                    ), &mut physics)
                    .with(Health { hp: 100.0 }, &mut healths)
                    .build();
            }
        }
    }
}
