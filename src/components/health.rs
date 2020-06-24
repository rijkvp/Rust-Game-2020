use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Default)]
pub struct Health
{
    pub hp: f32
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}