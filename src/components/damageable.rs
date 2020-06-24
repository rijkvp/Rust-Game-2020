use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default, Debug)]
pub struct Damageable {
    pub damage: f32,
    pub destroyed: bool
}

impl Component for Damageable {
    type Storage = DenseVecStorage<Self>;
}