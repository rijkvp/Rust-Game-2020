extern crate sdl2;

mod bullet;
mod button;
mod camera;
mod enemy;
mod event_manager;
mod physics;
mod player;
mod text;
mod texture_manager;
mod tile;
mod vectors;
mod world;

use crate::enemy::EnemyType;
use crate::bullet::{Bullet, BulletsManager};
use crate::button::Button;
use crate::camera::Camera;
use crate::event_manager::EventManager;
use crate::physics::*;
use crate::player::Player;
use crate::text::Text;
use crate::texture_manager::TextureManager;
use crate::vectors::Vector2;
use crate::world::World;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Instant;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const HUD_PADDING: i32 = 10;

enum GameState {
    MENU,
    GAME,
    GAMEOVER,
}

pub fn main() -> Result<(), String> {
    game()
}

fn game() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust Game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_man = TextureManager::new(canvas.texture_creator());
    let enemy_texture_1 = texture_man.get_texture(String::from("assets/textures/enemy1.bmp"));
    let enemy_texture_2 = texture_man.get_texture(String::from("assets/textures/enemy2.bmp"));

    let player_texture = texture_man.get_texture(String::from("assets/textures/player.bmp"));
    let bullet_texture = texture_man.get_texture(String::from("assets/textures/bullet.bmp"));
    let gun_texture = texture_man.get_texture(String::from("assets/textures/gun.bmp"));
    let logo_texture = texture_man.get_texture(String::from("assets/textures/logo.bmp"));

    let hud_text_top_left = Text::new(
        HUD_PADDING,
        HUD_PADDING,
        38,
        String::from("FLOOR 1"),
        &texture_man,
    );
    let hud_text_top_right = Text::new(
        HUD_PADDING,
        HUD_PADDING,
        38,
        String::from("4 LEFT"),
        &texture_man,
    );
    let mut hud_text_bottom_left = Text::new(
        HUD_PADDING,
        HUD_PADDING,
        46,
        String::from("100 HP"),
        &texture_man,
    );

    let mut evt_manager: EventManager = EventManager::new(sdl_context.event_pump()?);

    let mut game_state = GameState::MENU;
    let screen_center = Point::new(WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    let mut play_button = Button::new(
        Rect::from_center(screen_center - Point::new(0, 70), 200, 80),
        String::from("Play"),
        &texture_man,
    );
    let mut play_again_button = Button::new(
        Rect::from_center(screen_center - Point::new(0, 70), 400, 80),
        String::from("Play Again"),
        &texture_man,
    );
    let mut quit_button = Button::new(
        Rect::from_center(screen_center + Point::new(0, 70), 200, 80),
        String::from("Quit"),
        &texture_man,
    );

    // Gameplay elements
    let mut physics_manager = PhysicsManager::new();
    let mut player = Player::new(Vector2 { x: 0.0, y: 0.0 }, &mut physics_manager);
    let mut bullets_manager = BulletsManager::new();
    let mut fire_countdown = 0.0;

    let mut camera = Camera::new();
    let mut world = World::new(&texture_man);
    world.generate(&mut physics_manager);
    world.log_world();

    let mut delta_time: f32 = 0.0;
    let mut mouse_direction = Vector2::zero();
    'running: loop {
        let time = Instant::now();
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
                mouse_direction =
                    camera.screen_to_world(evt_manager.mouse_position) - player.position;
                player.update(&evt_manager, &mut physics_manager, delta_time);
                // let player_tile_pos = tile::world_to_tile_coords(
                //     player.position + Vector2 { x: 32.0, y: 32.0 },
                //     &world,
                // );
                //  println!("CHUNK: {}", player_tile_pos);
                //  world.set_surrounding(player_tile_pos);
                //  let surrounding = world.get_surrounding(player_tile_pos);
                //  println!("");
                // for (_y, row) in surrounding.iter().enumerate() {
                //     println!("");
                //     for (_x, col) in row.iter().enumerate() {
                //         print!("{}", col);
                //     }
                // }
                // println!("Player pos: {}", player.position);
                world.update_enemies(&mut player, &mut physics_manager, &mut bullets_manager, delta_time);
                if player.is_dead {
                    game_state = GameState::GAMEOVER;
                }
                bullets_manager.update_bullets(delta_time, &mut physics_manager, &mut player, &mut world);
                if fire_countdown > 0.0 {
                    fire_countdown -= delta_time;
                }
                if evt_manager.left_mouse_pressed && fire_countdown <= 0.0 {
                    fire_countdown = 0.1;
                    mouse_direction = mouse_direction.normalized();
                    let position_offset = mouse_direction * 50.0;
                    bullets_manager.add_bullet(Bullet::new(
                        player.position + position_offset,
                        mouse_direction,
                    ));
                }
                camera.update(player.position, delta_time);
                hud_text_bottom_left
                    .update((player.health as i32).to_string() + " HP", &texture_man)
            }
            GameState::GAMEOVER => {
                if play_again_button.is_pressed() {
                    game_state = GameState::MENU;
                }
                play_again_button.update(&evt_manager);
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
                canvas.copy(
                    &logo_texture,
                    None,
                    Rect::from_center(
                        screen_center - Point::new(0, 250),
                        (200.0 * 1.5) as u32,
                        (150.0 * 1.5) as u32,
                    ),
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
                for enemy in world.enemies.iter() {
                    match enemy.enemy_type
                    {
                        EnemyType::Range => {
                            canvas.copy(
                                &enemy_texture_2,
                                None,
                                Rect::from_center(camera.world_to_screen(enemy.position), 64, 64),
                            )?;
                        },
                        EnemyType::Melee => {
                            canvas.copy(
                                &enemy_texture_1,
                                None,
                                Rect::from_center(camera.world_to_screen(enemy.position), 64, 64),
                            )?;
                        },

                    }
                    
                }
                canvas.copy(
                    &player_texture,
                    None,
                    Rect::from_center(camera.world_to_screen(player.position), 64, 64),
                )?;
                canvas.copy_ex(
                    &gun_texture,
                    None,
                    Rect::from_center(camera.world_to_screen(player.position), 16, 64),
                    mouse_direction.get_degrees().into(),
                    Some(Point::new(8, 32)),
                    false,
                    false,
                )?;
                for bullet in bullets_manager.bullets.iter() {
                    canvas.copy_ex(
                        &bullet_texture,
                        None,
                        Rect::from_center(camera.world_to_screen(bullet.position), 8, 32),
                        bullet.get_rotation().into(),
                        Some(Point::new(4, 16)),
                        false,
                        false,
                    )?;
                }
                canvas.copy(
                    hud_text_top_left.get_texture(),
                    None,
                    hud_text_top_left.get_rect(),
                )?;
                let top_right_width: i32 = hud_text_top_right.get_rect().width() as i32;
                canvas.copy(
                    hud_text_top_right.get_texture(),
                    None,
                    Rect::new(
                        WINDOW_WIDTH as i32 - top_right_width - HUD_PADDING,
                        HUD_PADDING,
                        top_right_width as u32,
                        hud_text_top_right.get_rect().height(),
                    ),
                )?;
                let bottom_left_height: i32 = hud_text_bottom_left.get_rect().height() as i32;
                canvas.copy(
                    hud_text_bottom_left.get_texture(),
                    None,
                    Rect::new(
                        HUD_PADDING,
                        WINDOW_HEIGHT as i32 - bottom_left_height - HUD_PADDING,
                        hud_text_bottom_left.get_rect().width(),
                        bottom_left_height as u32,
                    ),
                )?;
            }
            GameState::GAMEOVER => {
                canvas.set_draw_color(play_again_button.get_color());
                canvas
                    .fill_rect(play_again_button.get_rect())
                    .map_err(|e| e.to_string())?;
                canvas.copy(
                    play_again_button.get_text_texture(),
                    None,
                    play_again_button.get_text_rect(),
                )?;
            }
        }
        canvas.present(); // Present the new frame

        delta_time = time.elapsed().as_secs_f32();
        // PRINT THE FPS
        // println!(
        //     "DT = {} ms ({} FPS)",
        //     (delta_time * 1000.0) as i32,
        //     (1.0 / delta_time) as i32
        // );
    }

    Ok(())
}
