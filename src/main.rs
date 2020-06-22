mod systems;
mod vectors;
mod components;
mod resources;

// States
mod menu;
mod game;

use crate::resources::Music;
use crate::menu::Menu;
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
    ui::{RenderUi, UiBundle},
    assets::HotReloadBundle,
    audio::AudioBundle,
    audio::DjSystemDesc,
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
                        .with_clear([0.212, 0.259, 0.404, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::MovementSystem, "movement_system", &["input_system"])
        .with(systems::PhysicsSystem, "physics_system", &["movement_system"])
        .with(systems::CameraFollowSystem, "camera_system", &["physics_system"])
        .with(systems::PlayerCombatSystem, "player_combat_system", &["input_system"])
        .with(systems::HealthSystem, "health_system", &["physics_system"])
        .with(systems::DestroySystem, "destroy_system", &["physics_system"])
        .with(systems::LifetimeSystem, "lifetime_system", &[])
        .with(systems::AISystem, "ai_system", &[])
        .with(systems::AICombatSystem, "ai_combat_system", &["ai_system"]);

    let mut game = Application::new(assets_dir, Menu::default(), game_data)?;
    game.run();

    Ok(())
}
