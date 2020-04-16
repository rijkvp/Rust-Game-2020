use sdl2::render::Texture;
use crate::event_manager::EventManager;
use crate::texture_manager::TextureManager;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

enum ButtonState {
    NORMAL,
    HOVER,
    PRESSED,
}

pub struct Button<'a> {
    rect: Rect,
    text_rect: Rect,
    state: ButtonState,
    text_texture: &'a Texture<'a>,
}

const FONT_FILE: &str = "assets/fonts/Oxanium-Bold.ttf";
const PADDING: u16 = 12;

impl<'a> Button<'a> {
    pub fn new(rect: Rect, text: String, texture_manager: &'a TextureManager) -> Button<'a> {
        let text_tex: Texture<'a> = texture_manager.create_font_texture(
            String::from(FONT_FILE),
            text,
            rect.height() as u16 - PADDING,
            Color::RGBA(255, 255, 255, 255)
        );
        let TextureQuery { width, height, .. } = text_tex.query();
        let text_rect = Rect::from_center(rect.center(), width, height);
        Button {
            rect,
            text_rect,
            state: ButtonState::NORMAL,
            text_texture: text_tex,
        }
    }

    pub fn is_pressed(&self) -> bool {
        match self.state
        {
            ButtonState::PRESSED => true,
            _ => false
        }
    }

    pub fn update(&mut self, event_manager: &EventManager) {               
        if event_manager.mouse_in_rect(self.rect) {
            if event_manager.left_mouse_pressed {
                self.state = ButtonState::PRESSED;
            } else {
                self.state = ButtonState::HOVER;
            }
        } else {
            self.state = ButtonState::NORMAL;
        }
    }

    pub fn get_color(&self) -> Color {
        match self.state {
            ButtonState::NORMAL => {
                Color::RGBA(100, 100, 100, 255)
            }
            ButtonState::HOVER => {
                Color::RGBA(50, 50, 50, 255)
            }
            ButtonState::PRESSED => {
                Color::RGBA(10, 10, 10, 255)
            }
        }
    }

    pub fn get_rect(&self) -> Rect
    {
        self.rect
    }

    pub fn get_text_rect(&self) -> Rect
    {
        self.text_rect
    }

    pub fn get_text_texture(&self) -> &Texture
    {
        return &self.text_texture;
    }
}
