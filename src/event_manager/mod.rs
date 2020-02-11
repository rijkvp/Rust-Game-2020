use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
        }
    }

    pub fn update_events(&mut self)
    {
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
                Event::MouseMotion {..} => {
                    
                },
                _e => {
                 //   println!("{:?}", e);
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
        let mut y = {
            if self.move_up && self.move_down { 0.0 }
            else if self.move_up { 1.0 }
            else if self.move_down { -1.0 }
            else { 0.0 }
        };
        y = -y;
        Vector2 { x, y }
    }
}