extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rust Game", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
                    
    println!("This example simply prints all events SDL knows about.");
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .map_err(|e| e.to_string())?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion {..} => {},
                e => {
                    println!("{:?}", e);
                }
            }
        }

        canvas.clear();
        let angle = 0.0;

        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            texture_canvas.fill_rect(Rect::new(100, 100, 64, 64)).expect("could not fill rect");
        }).map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGBA(100, 100, 100, 255));
        let dst = Some(Rect::new(0, 0, 400, 300));
        canvas.clear();
        canvas.copy_ex(&texture,
            None,
            dst,
            angle,
            Some(Point::new(400, 300)),
            false,
            false
        )?;
        canvas.present()
    }

    Ok(())
}