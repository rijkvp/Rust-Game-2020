use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Copy, Clone)]
pub enum EnemyType
{
    Melee, 
    Range,
}

impl Default for EnemyType {
    fn default() -> Self {
        EnemyType::Melee
    }
}

#[derive(Debug, Default)]
pub struct Enemy
{
    enemy_type: EnemyType
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}