use crate::vectors::Vector2;

pub struct Enemy
{
    pub position: Vector2
}

impl Enemy
{
    pub fn update(&mut self, target: Vector2)
    {
        let mut dir = target - self.position;
        if dir.magnitude() > 64.0
        {
            dir = dir.normalized();
            let enemy_movement = dir * 100.0 * crate::DELTA_TIME;
            self.position = self.position + enemy_movement;
        }
    }
}
