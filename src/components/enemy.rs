use amethyst::ecs::prelude::{Component, DenseVecStorage};
use rand::Rng;

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
    pub enemy_type: EnemyType,
    pub can_attack: bool,
    pub fire_timer: f32,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

impl Enemy {
    pub fn random() -> Enemy {
        let mut rng = rand::thread_rng();
        let enemy_type =  match rng.gen_range(0, 2) {
            0 => EnemyType::Melee,
            1 => EnemyType::Range,
            _ => panic!("Not all possibilities added!")
        };
        Self {
            enemy_type,
            can_attack: false,
            fire_timer: 0.0,
        }
    }
}