use std::path::Path;

use sdl2::pixels::Color;

pub struct TextureManager
{
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ttf_context: sdl2::ttf::Sdl2TtfContext,
}

impl TextureManager
{
    pub fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> TextureManager
    {
        let temp_ttf_context = sdl2::ttf::init().map_err(|e| e.to_string());
        let ttf_context = match temp_ttf_context {
            Ok(ttf_context) => ttf_context,
            Err(error) => {
                panic!("Problem when creating TTF context: {:?}", error)
            },
        };
        TextureManager
        {
            texture_creator,
            ttf_context,
        }
    }

    pub fn get_texture(&self, path: String) -> sdl2::render::Texture<'_>
    {
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(&path));
        let surface = match temp_surface {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening a file: {:?}", error)
            },
        };
        let temp_texture = self.texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string());
        let texture = match temp_texture {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem creating texutre surface: {:?}", error)
            },
        };
        return texture;
    }

    pub fn create_font_texture(&self, path: String, text: String, size: u16, color: Color) -> sdl2::render::Texture<'_>
    {
        let temp_font = self.ttf_context.load_font(path, size);
        let mut font = match temp_font {
            Ok(font) => font,
            Err(error) => {
                panic!("Problem while loading font: {:?}", error)
            },
        };
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        let temp_surface = font.render(&text).blended(color).map_err(|e| e.to_string());
        let surface = match temp_surface {
            Ok(surface) => surface,
            Err(error) => {
                panic!("Problem creating font surface: {:?}", error)
            },
        };
        let temp_texture = self.texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string());
        let texture = match temp_texture {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem creating texutre surface: {:?}", error)
            },
        };
        return texture;
    }
}