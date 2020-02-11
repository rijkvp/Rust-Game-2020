extern crate sdl2;

mod enemy;
mod event_manager;
mod player;
mod texture_manager;
mod vectors;

use crate::enemy::Enemy;
use crate::event_manager::EventManager;
use crate::player::Player;
use crate::texture_manager::TextureManager;
use crate::vectors::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const FPS: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / FPS;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Game", 1000, 800)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_man = TextureManager {
        texture_creator: canvas.texture_creator(),
    };
    let enemy_texture = texture_man.get_texture(String::from("assets/enemy1.bmp"));
    let player_texture = texture_man.get_texture(String::from("assets/player.bmp"));
    let mut evt_manager: EventManager = EventManager::new(sdl_context.event_pump()?);

    let mut player = Player {
        position: Vector2 { x: 500.0, y: 500.0 },
    };

    let mut enemies = [ Enemy {
        position: Vector2 { x: 400.0, y: 600.0 },
    }, Enemy {
        position: Vector2 { x: 800.0, y: 800.0 },
    } ];

    'running: loop {
        // Events
        evt_manager.update_events();
        if evt_manager.quit {
            break 'running;
        }

        // Update
        player.update(&evt_manager);
        for enemy in enemies.iter_mut()
        {
            enemy.update(player.position);
        }

        // Draw
        canvas.set_draw_color(Color::RGBA(180, 180, 180, 255));
        canvas.clear();

        // Old player drawing rect
        // canvas.set_draw_color(Color::RGBA(255, 50, 50, 255));
        // let player_rect = Rect::from_center(player.position.to_point(), 64, 64);
        // canvas.fill_rect(player_rect).map_err(|e| e.to_string())?;

        for enemy in enemies.iter()
        {
            canvas.copy(
                &enemy_texture,
                None,
                Rect::from_center(enemy.position.to_point(), 64, 64),
            )?;
        }
        canvas.copy(
            &player_texture,
            None,
            Rect::from_center(player.position.to_point(), 64, 64),
        )?;
        canvas.present();

        canvas.present();

        // Wait for next frame
        std::thread::sleep(Duration::from_secs_f32(DELTA_TIME));
    }

    Ok(())
}
