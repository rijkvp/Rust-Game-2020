use crate::vectors::Vector2;
use amethyst::core::Transform;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Game,
    GameOver
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}

#[derive(Default)]
pub struct GameInfo 
{
    pub game_state: GameState,
    pub camera_transform: Transform,
    pub player_position: Vector2,
    pub in_game: bool,
    pub wave: u16,
    pub score: u32,
}