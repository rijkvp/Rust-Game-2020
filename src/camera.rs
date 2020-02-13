use crate::vectors::Vector2;
use sdl2::rect::Point;

pub struct Camera {
    position: Vector2,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector2::zero(),
        }
    }

    pub fn update(&mut self, target: Vector2) {               
        let new_position = Vector2::lerp(self.position, target, crate::DELTA_TIME * 1.6);
        let distance = (new_position - self.position).magnitude();
        if distance >= 1.0
        {
            self.position = new_position;
        }
    }

    pub fn world_to_screen(&self, world_coords: Vector2) -> Point {
        Point::new(
            (world_coords.x + (crate::WINDOW_WIDTH as f32 / 2.0) - self.position.x) as i32,
            -(world_coords.y - (crate::WINDOW_HEIGHT as f32 / 2.0) - self.position.y) as i32,
        )
    }

    pub fn screen_to_world(&self, screen_coords: Vector2) -> Vector2 {
       Vector2{
           x: screen_coords.x - (crate::WINDOW_WIDTH as f32 / 2.0) + self.position.x,
           y: -(screen_coords.y - (crate::WINDOW_HEIGHT as f32 / 2.0) - self.position.y),
       }
    }
}
