use crate::vectors::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub enum PhysicsType {
    Static,
    Dynamic,
}

impl Default for PhysicsType {
    fn default() -> Self {
        PhysicsType::Static
    }
}

#[derive(Default, Debug)]
pub struct Physics {
    pub physics_type: PhysicsType,
    pub velocity: Vector2,
    pub drag: bool,
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}