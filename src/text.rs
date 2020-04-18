use crate::texture_manager::TextureManager;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;

pub struct Text<'a>
{
    text_rect: Rect,
    text_texture: Texture<'a>,
    size: u16,
}

const FONT_FILE: &str = "assets/fonts/Oxanium-Regular.ttf";
const PADDING: u16 = 12;

impl<'a> Text<'a>
{
    pub fn new(x: i32, y: i32, max_height: u16, text: String, texture_manager: &'a TextureManager) -> Text<'a> {
        let size = max_height - PADDING;
        let text_tex = texture_manager.create_font_texture(
            String::from(FONT_FILE),
            text,
            size,
            Color::RGBA(255, 255, 255, 255)
        );
        let TextureQuery { width, height, .. } = text_tex.query();
        let text_rect = Rect::new(x, y, width, height);
        Text {
            text_rect,
            text_texture: text_tex,
            size,
        }
    }

    pub fn update(&mut self, text: String, texture_manager: &'a TextureManager)
    {
        let new_texture: Texture<'a> = texture_manager.create_font_texture(
            String::from(FONT_FILE),
            text,
            self.size,
            Color::RGBA(255, 255, 255, 255)
        );
        self.text_texture = new_texture;
        let TextureQuery { width, height, .. } = self.text_texture.query();
        self.text_rect = Rect::new(self.text_rect.x, self.text_rect.y, width, height);
    }

    pub fn get_rect(&self) -> Rect
    {
        self.text_rect
    }

    pub fn get_texture(&self) -> &Texture
    {
        return &self.text_texture;
    }
}