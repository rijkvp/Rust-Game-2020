use crate::vectors::Vector2;
use amethyst::core::Transform;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    _Game,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}

#[derive(Default)]
pub struct GameInfo {
    pub game_state: GameState,
    pub camera_transform: Transform,
    pub player_position: Vector2,
    pub in_game: bool,
    pub wave: u16,
    pub score: u32,
}

impl GameInfo {
    pub fn get_wave_multiplier(&self) -> f32 {
        if self.wave > 4 {
            4.0
        } else {
            self.wave as f32
        }
    }
}
