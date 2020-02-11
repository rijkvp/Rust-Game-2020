use crate::vectors::Vector2;
use sdl2::rect::Point;

pub struct Camera {
    position: Vector2,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self, target: Vector2) {               
        self.position = Vector2::lerp(self.position, target, crate::DELTA_TIME * 3.0);
    }

    pub fn world_to_screen(&self, world_coords: Vector2) -> Point {
        return Point::new(
            (world_coords.x + (crate::WINDOW_WIDTH as f32 / 2.0) - self.position.x) as i32,
            -(world_coords.y - (crate::WINDOW_HEIGHT as f32 / 2.0) - self.position.y) as i32,
        );
    }

    // TODO: Create screen_to_world() metod
}
