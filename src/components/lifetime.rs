use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Default)]
pub struct Lifetime {
    pub lifetime: f32 
}

impl Component for Lifetime {
    type Storage = DenseVecStorage<Self>;
}