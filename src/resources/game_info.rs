use crate::vectors::Vector2;
use amethyst::core::Transform;

#[derive(Default)]
pub struct GameInfo 
{
    pub camera_transform: Transform,
    pub player_position: Vector2
}