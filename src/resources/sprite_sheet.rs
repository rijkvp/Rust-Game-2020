use amethyst::assets::Handle;
use amethyst::renderer::*;


pub struct SpriteSheetHolder
{
    pub sprite_sheet: Option<Handle<SpriteSheet>>
}

impl Default for SpriteSheetHolder {
    fn default() -> Self {
        Self{
            sprite_sheet: None
        }
    }
}