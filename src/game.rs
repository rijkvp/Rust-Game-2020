use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use rand::{thread_rng, Rng};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const ENEMY_COUNT: u16 = 10;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);
        // Register components
        // NOTE: Not needed anymore when used by systems
        world.register::<Player>();
        world.register::<Enemy>();

        initialise_players(world, sprite_sheet_handle);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_players(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut player_transform = Transform::default();

    player_transform.set_scale(Vector3::new(0.2, 0.2, 0.0));
    world
        .create_entity()
        .with(Player)
        .with(player_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        })
        .build();

    let mut rng = rand::thread_rng();
    for _i in 0..ENEMY_COUNT {
        let mut enemy_transform = Transform::default();
        enemy_transform.set_scale(Vector3::new(0.2, 0.2, 0.0));
        enemy_transform.set_translation_xyz(
            rng.gen_range(-ARENA_WIDTH / 2.0, ARENA_WIDTH / 2.0),
            rng.gen_range(-ARENA_HEIGHT / 2.0, ARENA_HEIGHT / 2.0),
            0.0,
        );
        world
            .create_entity()
            .with(Enemy)
            .with(enemy_transform)
            .with(SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 1,
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
