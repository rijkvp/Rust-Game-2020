use crate::resources::initialise_audio;
use crate::components::*;
use crate::resources::{GameInfo, GameState, SpriteSheetHolder};
use crate::vectors::Vector2;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, transform::Transform},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use rand::Rng;
use crate::game_over::GameOver;

pub const ARENA_HEIGHT: f32 = 1080.0;
pub const ARENA_WIDTH: f32 = 1920.0;
pub const ENEMY_COUNT: u16 = 20;

#[derive(Default)]
pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet = load_sprite_sheet(world);
        world.insert(SpriteSheetHolder {
            sprite_sheet: Some(sprite_sheet.clone()),
        });

        initialise_camera(world);
        // Register components
        // NOTE: Not needed anymore when used by systems
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Physics>();
        world.register::<Damageable>();
        world.register::<Lifetime>();

        initialise_players(world, sprite_sheet);
        initialise_audio(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let game_info = data.world.read_resource::<GameInfo>();
        match game_info.game_state {
            GameState::GameOver => Trans::Switch(Box::new(GameOver::default())),
            _ => Trans::None,
        }
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH / 4.0, ARENA_HEIGHT / 4.0))
        .with(transform)
        .build();
}

fn initialise_players(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    
    // Background
    let mut bg_transform = Transform::default();
    bg_transform.set_scale(Vector3::new(5.0, 5.0, 0.0));
    bg_transform.set_translation_z(-100.0);
    world.create_entity()
    .with(bg_transform)
    .with(SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 3,
    }).build();
    
    // Player
    let mut player_transform = Transform::default();
    player_transform.set_scale(Vector3::new(0.2, 0.2, 0.0));
    world
        .create_entity()
        .with(Player::default())
        .with(player_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        })
        .with(Physics::with_id(
            PhysicsType::Dynamic,
            PhysicsLayer::None,
            Vector2::default(),
            true,
            1
        ))
        .with(Health{hp: 100.0})
        .build();

    world.insert(GameInfo::default());

    let mut rng = rand::thread_rng();
    for i in 0..ENEMY_COUNT {
        let mut enemy_transform = Transform::default();
        enemy_transform.set_scale(Vector3::new(0.2, 0.2, 0.0));
        enemy_transform.set_translation_xyz(
            rng.gen_range(-ARENA_WIDTH / 4.0, ARENA_WIDTH / 4.0),
            rng.gen_range(-ARENA_HEIGHT / 4.0, ARENA_HEIGHT / 4.0),
            0.0,
        );
        world
            .create_entity()
            .with(Enemy::random())
            .with(enemy_transform)
            .with(SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 1,
            })
            .with(Physics::with_id(
                PhysicsType::Dynamic,
                PhysicsLayer::None,
                Vector2::default(),
                true,
                2 + i
            ))
            .with(Health {
                hp: 100.0
            })
            .build();
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
