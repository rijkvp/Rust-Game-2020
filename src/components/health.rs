use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Default)]
pub struct Health
{
    pub hp: f32,
    pub is_dead: bool
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}