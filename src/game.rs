extern crate sdl2;

mod enemy;
mod event_manager;
mod player;
mod texture_manager;
mod vectors;
mod camera;
mod physics;

use crate::enemy::Enemy;
use crate::event_manager::EventManager;
use crate::player::Player;
use crate::texture_manager::TextureManager;
use crate::vectors::Vector2;
use crate::camera::Camera;
use crate::physics::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const FPS: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / FPS;
const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 800;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem
    .window("Rust Game", WINDOW_WIDTH, WINDOW_HEIGHT)
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
    let bg_texture = texture_man.get_texture(String::from("assets/test_bg.bmp"));
    
    
    let mut evt_manager: EventManager = EventManager::new(sdl_context.event_pump()?);
    
    
    let mut physics_manager = PhysicsManager::new();

    let mut player = Player::new(Vector2{x: 0.0,y: 0.0}, &mut physics_manager);

    let mut enemies = [ Enemy::new(Vector2{ x: 400.0, y: 100.0}, &mut physics_manager), 
    Enemy::new(Vector2{ x: -600.0, y: 200.0}, &mut physics_manager),
    Enemy::new(Vector2{ x: 200.0, y: -500.0}, &mut physics_manager),
    Enemy::new(Vector2{ x: -100.0, y: -300.0}, &mut physics_manager) ];
    
    let mut camera = Camera::new();

    'running: loop {
        // Events
        evt_manager.update_events();
        if evt_manager.quit {
            break 'running;
        }

        // Update
        player.update(&evt_manager, &mut physics_manager);
        for enemy in enemies.iter_mut()
        {
            enemy.update(player.position, &mut physics_manager);
        }

        camera.update(player.position);
        //physics_manager.log_colliders();

        // Draw
        canvas.set_draw_color(Color::RGBA(180, 180, 180, 255));
        canvas.clear(); // Clear the previous frame

        // Old player drawing rect
        // canvas.set_draw_color(Color::RGBA(255, 50, 50, 255));
        // let player_rect = Rect::from_center(player.position.to_point(), 64, 64);
        // canvas.fill_rect(player_rect).map_err(|e| e.to_string())?;
        canvas.copy(
            &bg_texture,
            None,
            Rect::from_center(camera.world_to_screen(Vector2{x: 0.0, y: 0.0}), crate::WINDOW_WIDTH, crate::WINDOW_HEIGHT),
        )?;
        for enemy in enemies.iter()
        {
            canvas.copy(
                &enemy_texture,
                None,
                Rect::from_center(camera.world_to_screen(enemy.position), 64, 64),
            )?;
        }
        canvas.copy(
            &player_texture,
            None,
            Rect::from_center(camera.world_to_screen(player.position), 64, 64),
        )?;

        canvas.present(); // Present the new frame

        // Wait for next frame
        std::thread::sleep(Duration::from_secs_f32(DELTA_TIME));
    }

    Ok(())
}
