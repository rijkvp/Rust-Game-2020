use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}