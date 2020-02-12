use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::vectors;
use crate::vectors::Vector2;

pub struct EventManager
{
    event_pump: EventPump,
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
    pub quit: bool,
    pub mouse_position: Vector2,
    pub left_mouse_pressed: bool,
}

impl EventManager
{
    pub fn new(event_pump: EventPump) -> EventManager {
        EventManager {
            event_pump,
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
            quit: false,
            mouse_position: Vector2::zero(),
            left_mouse_pressed: false,
        }
    }

    pub fn update_events(&mut self)
    {
        let mousestate = self.event_pump.mouse_state();
        self.mouse_position = Vector2{x: mousestate.x() as f32, y: mousestate.y() as f32};
        self.left_mouse_pressed = mousestate.is_mouse_button_pressed(MouseButton::Left);
        
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.quit = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::KeyDown { keycode: Some(Keycode::A), .. }  => {
                    self.move_left = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } | Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    self.move_left = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. }  | Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    self.move_right = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. }  | Event::KeyUp { keycode: Some(Keycode::D), .. }=> {
                    self.move_right = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    self.move_up = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } | Event::KeyUp { keycode: Some(Keycode::W), .. }=> {
                    self.move_up = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    self.move_down = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } | Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    self.move_down = false;
                },
                _e => {
                 //   println!("{:?}", _e);
                }
            }
        }
    }

    pub fn get_input_vector(&self) -> vectors::Vector2
    {
        let x = {
            if self.move_right && self.move_left { 0.0 }
            else if self.move_right { 1.0 }
            else if self.move_left { -1.0 }
            else { 0.0 }
        };
        let y = {
            if self.move_up && self.move_down { 0.0 }
            else if self.move_up { 1.0 }
            else if self.move_down { -1.0 }
            else { 0.0 }
        };
        Vector2 { x, y }
    }

    pub fn mouse_in_rect(&self, rect: sdl2::rect::Rect) -> bool
    {
        (self.mouse_position.x as i32 > rect.x && (self.mouse_position.x as i32) < (rect.x + rect.width() as i32)
        && self.mouse_position.y as i32 > rect.y && (self.mouse_position.y as i32) < (rect.y + rect.height() as i32))
    }
}