use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Player {
    pub fire_timer: f32 
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}