use crate::resources::{GameInfo, GameState};
use crate::resources::{initialise_audio};
use amethyst::{
    ecs::prelude::Entity,
    input::is_close_requested,
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};
use crate::menu::Menu;
use crate::game::Game;

const BUTTON_PLAY_AGAIN: &str = "play_again";
const BUTTON_MENU: &str = "menu";

#[derive(Default, Debug)]
pub struct GameOver {
    ui_root: Option<Entity>,
    button_play_again: Option<Entity>,
    button_menu: Option<Entity>,
}

impl SimpleState for GameOver {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // create UI from prefab and save the reference.
        let world = data.world;
        initialise_audio(world);
        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game_over.ron", ())));

        let mut game_info = world.write_resource::<GameInfo>();
        game_info.game_state = GameState::Menu;
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_play_again.is_none() || self.button_menu.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_play_again = ui_finder.find(BUTTON_PLAY_AGAIN);
                self.button_menu = ui_finder.find(BUTTON_MENU);
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
                if Some(target) == self.button_play_again {
                    return Trans::Switch(Box::new(Game::default()));
                } else if Some(target) == self.button_menu {
                    return Trans::Switch(Box::new(Menu::default()));
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
        self.button_play_again = None;
        self.button_menu = None;
    }
}
