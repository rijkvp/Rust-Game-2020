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
    pub id: u16
}

impl Physics {
    pub fn simple(physics_type: PhysicsType, layer: PhysicsLayer, velocity: Vector2) -> Self
    {
        Self {
            physics_type,
            layer,
            velocity,
            drag: false,
            id :0
        }
    }

    pub fn with_id(physics_type: PhysicsType, layer: PhysicsLayer, start_velocity: Vector2, drag: bool, id: u16) -> Self
    {
        Self {
            physics_type,
            layer,
            velocity: start_velocity,
            drag,
            id
        }
    }
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}