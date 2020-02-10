extern crate sdl2;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use sdl2::pixels::{Color, PixelFormatEnum};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
   
    let window = video_subsystem.window("Rust Game", 1000, 800)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/enemy1.bmp"))?;
    let texture = texture_creator.create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut player_point = Point::new(500, 500);
    let mut enemy_point = Point::new(200, 200);

    let mut move_left = false;
    let mut move_right = false;
    const FPS: f64 = 60.0;
    const DELTA_TIME: f64 = 1.0 / FPS;

    'running: loop {
        // Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    move_left = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    move_left = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    move_right = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    move_right = false;
                },
                Event::MouseMotion {..} => {},
                e => {
                    println!("{:?}", e);
                }
            }
        }

        // Update
        if move_right {
            player_point.x += (200.0 * DELTA_TIME) as i32;
        }
        else if move_left {
            player_point.x -= (200.0 * DELTA_TIME) as i32;
        }


        canvas.set_draw_color(Color::RGBA(80, 80, 80, 255));
        canvas.clear();

        let player_rect = Rect::from_center(player_point, 64, 64);
        canvas.set_draw_color(Color::RGBA(255, 50, 50, 255));
        canvas.fill_rect(player_rect).map_err(|e| e.to_string())?;

        let mut dirX: f64 = (player_point.x - enemy_point.x) as f64;
        let mut dirY: f64 = (player_point.y - enemy_point.y) as f64;
        use std::num;
        let sqr: f64 = dirX * dirX + dirY * dirY;
        let mag = sqr.sqrt();
        dirX /= mag;
        dirY /= mag;
        dirX *= 100.0 * DELTA_TIME;
        dirY *= 100.0 * DELTA_TIME;
        enemy_point.x += dirX as i32;
        enemy_point.y += dirY as i32;

        canvas.copy(&texture, None, Rect::from_center(enemy_point, 64, 64))?;
        canvas.present();

        canvas.present();

        // Wait for next frame
        std::thread::sleep(Duration::from_secs_f64(DELTA_TIME));
    };

    Ok(())
}