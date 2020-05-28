mod game;
mod systems;
mod vectors;
mod components;
mod resources;

use crate::game::Game;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");

    let assets_dir = app_root.join("assets/");

    // Load input bundle
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::MovementSystem, "movement_system", &["input_system"])
        .with(systems::PhysicsSystem, "physics_system", &["movement_system"])
        .with(systems::CameraFollowSystem, "camera_system", &["physics_system"])
        .with(systems::PlayerCombat, "player_combat_system", &["input_system"])
        .with(systems::LifetimeSystem, "lifetime_system", &[]);

    let mut game = Application::new(assets_dir, Game, game_data)?;
    game.run();

    Ok(())
}
