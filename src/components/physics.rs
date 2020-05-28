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

#[derive(Debug, Copy, Clone)]
pub enum PhysicsLayer {
    None,
    Projectile,
}

impl Default for PhysicsLayer {
    fn default() -> Self { 
        PhysicsLayer::None
    }
}

impl PhysicsLayer {
    pub fn collidable(layer: PhysicsLayer) -> bool
    {
        match layer {
            PhysicsLayer::None => true,
            PhysicsLayer::Projectile => false,
        }
    }
}

#[derive(Default, Debug)]
pub struct Physics {
    pub physics_type: PhysicsType,
    pub layer: PhysicsLayer,
    pub velocity: Vector2,
    pub drag: bool,
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}