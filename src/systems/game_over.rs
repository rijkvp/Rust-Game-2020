use crate::components::{Health, Player};
use crate::resources::{GameInfo, GameState};
use amethyst::ecs::{Join, ReadStorage, System, Write};

pub struct GameOverSystem;

impl<'s> System<'s> for GameOverSystem {
    type SystemData = (
        ReadStorage<'s, Health>,
        ReadStorage<'s, Player>,
        Write<'s, GameInfo>,
    );

    fn run(&mut self, (healths, players, mut game_info): Self::SystemData) {
        for (health, _player) in (&healths, &players).join() {
            if health.hp <= 0.0 {
                game_info.game_state = GameState::GameOver;
            }
        }
    }
}
