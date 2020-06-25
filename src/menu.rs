use crate::resources::{GameInfo, GameState};
use crate::resources::{initialise_audio};
use amethyst::{
    ecs::prelude::Entity,
    input::is_close_requested,
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use crate::game::Game;

const BUTTON_PLAY: &str = "play";
const BUTTON_QUIT: &str = "quit";

#[derive(Default, Debug)]
pub struct Menu {
    ui_root: Option<Entity>,
    button_play: Option<Entity>,
    button_quit: Option<Entity>,
}

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // create UI from prefab and save the reference.
        let world = data.world;
        initialise_audio(world);
        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));

        let mut game_info = world.write_resource::<GameInfo>();
        game_info.game_state = GameState::Menu;
        game_info.in_game = false;
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_play.is_none() || self.button_quit.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_play = ui_finder.find(BUTTON_PLAY);
                self.button_quit = ui_finder.find(BUTTON_QUIT);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_play {
                    return Trans::Switch(Box::new(Game::default()));
                } else if Some(target) == self.button_quit {
                    return Trans::Quit;
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Menu");
        }

        self.ui_root = None;
        self.button_play = None;
        self.button_quit = None;
    }
}
