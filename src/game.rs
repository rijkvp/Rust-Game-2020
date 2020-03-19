extern crate sdl2;

mod bullet;
mod button;
mod camera;
mod enemy;
mod event_manager;
mod physics;
mod player;
mod texture_manager;
mod tile;
mod vectors;
mod world;

use crate::bullet::Bullet;
use crate::button::Button;
use crate::camera::Camera;
use crate::enemy::Enemy;
use crate::event_manager::EventManager;
use crate::physics::*;
use crate::player::Player;
use crate::texture_manager::TextureManager;
use crate::vectors::Vector2;
use crate::world::World;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;

const FPS: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / FPS;
const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 800;

enum GameState {
    MENU,
    GAME,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust Game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_man = TextureManager::new(canvas.texture_creator());

    let enemy_texture = texture_man.get_texture(String::from("assets/enemy1.bmp"));
    let player_texture = texture_man.get_texture(String::from("assets/player.bmp"));

    let mut evt_manager: EventManager = EventManager::new(sdl_context.event_pump()?);
    let mut physics_manager = PhysicsManager::new();

    let mut player = Player::new(Vector2 { x: 0.0, y: 0.0 }, &mut physics_manager);

    let mut enemies = Vec::<Enemy>::new();
    enemies.push(Enemy::new(
        Vector2 { x: 400.0, y: 100.0 },
        &mut physics_manager,
    ));
    enemies.push(Enemy::new(
        Vector2 {
            x: -600.0,
            y: 200.0,
        },
        &mut physics_manager,
    ));
    enemies.push(Enemy::new(
        Vector2 {
            x: 200.0,
            y: -500.0,
        },
        &mut physics_manager,
    ));
    enemies.push(Enemy::new(
        Vector2 {
            x: -100.0,
            y: -300.0,
        },
        &mut physics_manager,
    ));

    let mut bullets = Vec::<Bullet>::new();
    let mut fire_countdown = 0.0;

    let mut camera = Camera::new();

    let mut game_state = GameState::MENU;
    let screen_center = Point::new(WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    let mut play_button = Button::new(
        Rect::from_center(screen_center - Point::new(0, 70), 200, 80),
        String::from("Play"),
        &texture_man,
    );
    let mut quit_button = Button::new(
        Rect::from_center(screen_center + Point::new(0, 70), 200, 80),
        String::from("Quit"),
        &texture_man,
    );
    let mut world = World::new(&texture_man);
    world.generate();
    world.log_world();

    println!("Testing");
    println!("0,0 : {}", tile::tile_to_world_coords(0, 0, &world));
    println!("50,50 : {}", tile::tile_to_world_coords(50, 50, &world));

    'running: loop {
        // Events
        evt_manager.update_events();
        if evt_manager.quit {
            break 'running;
        }

        // Update
        match game_state {
            GameState::MENU => {
                if play_button.is_pressed() {
                    game_state = GameState::GAME;
                }
                if quit_button.is_pressed() {
                    break 'running;
                }
                play_button.update(&evt_manager);
                quit_button.update(&evt_manager);
            }
            GameState::GAME => {
                player.update(&evt_manager, &mut physics_manager);
                let player_tile_pos = tile::world_to_tile_coords(player.position  + Vector2{x: 32.0, y: 32.0}, &world);
                //println!("CHUNK: {}", player_tile_pos);
                //world.set_surrounding(player_tile_pos);
                // let surrounding = world.get_surrounding(player_tile_pos);
                // println!("");
                // for (_y, row) in surrounding.iter().enumerate() {
                //     println!("");
                //     for (_x, col) in row.iter().enumerate() {
                //         print!("{}", col);
                //     }
                // }
                // println!("Player pos: {}", player.position);
                for enemy in enemies.iter_mut() {
                    enemy.update(player.position, &mut physics_manager);
                }
                enemies.retain(|enemy| !enemy.is_dead);
                for bullet in bullets.iter_mut() {
                    bullet.update(&mut physics_manager, &mut enemies);
                }
                bullets.retain(|bullet| !bullet.is_destroyed);
                if fire_countdown > 0.0 {
                    fire_countdown -= crate::DELTA_TIME;
                }
                if evt_manager.left_mouse_pressed && fire_countdown <= 0.0 {
                    fire_countdown = 0.1;
                    let mut direction =
                        camera.screen_to_world(evt_manager.mouse_position) - player.position;
                    direction = direction.normalized();
                    let position_offset = direction * 50.0;
                    bullets.push(Bullet::new(player.position + position_offset, direction));
                }
                camera.update(player.position);
            }
        }

        // Draw
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear(); // Clear the previous frame

        match game_state {
            GameState::MENU => {
                canvas.set_draw_color(play_button.get_color());
                canvas
                    .fill_rect(play_button.get_rect())
                    .map_err(|e| e.to_string())?;
                canvas.copy(
                    play_button.get_text_texture(),
                    None,
                    play_button.get_text_rect(),
                )?;
                canvas.set_draw_color(quit_button.get_color());
                canvas
                    .fill_rect(quit_button.get_rect())
                    .map_err(|e| e.to_string())?;
                canvas.copy(
                    quit_button.get_text_texture(),
                    None,
                    quit_button.get_text_rect(),
                )?;
            }
            GameState::GAME => {
                // Draw tiles
                for x in 0..world.get_size() {
                    for y in 0..world.get_size() {
                        canvas.copy(
                            world.get_texture(world.get_tile(x, y)),
                            None,
                            Rect::from_center(
                                tile::tile_to_screen_coords(x as u16, y as u16, &camera, &world),
                                64,
                                64,
                            ),
                        )?;
                    }
                }
                for enemy in enemies.iter() {
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
                for bullet in bullets.iter() {
                    canvas.set_draw_color(Color::RGBA(255, 255, 0, 255));
                    canvas
                        .fill_rect(Rect::from_center(
                            camera.world_to_screen(bullet.position),
                            10,
                            10,
                        ))
                        .map_err(|e| e.to_string())?;
                }
            }
        }

        canvas.present(); // Present the new frame

        // Wait for next frame
        std::thread::sleep(Duration::from_secs_f32(DELTA_TIME));
    }

    Ok(())
}
