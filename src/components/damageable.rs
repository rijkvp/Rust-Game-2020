use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, PartialEq)]
pub enum DamageType {
    Player,
    Enemy,
}

impl Default for DamageType {
    fn default() -> Self {
        DamageType::Player
    }
}

#[derive(Default, Debug)]
pub struct Damageable {
    pub damage: f32,
    pub destroyed: bool,
    pub damage_type: DamageType
}

impl Component for Damageable {
    type Storage = DenseVecStorage<Self>;
}