use sdl2::render::Texture;
use crate::texture_manager::TextureManager;

pub struct TileInfo<'r>
{
    id: u16,
    pub texture: Texture<'r>,
    pub is_solid: bool
}

impl TileInfo<'_> {
    pub fn new(id: u16, texture_path: String, is_solid: bool, texture_manager: &TextureManager) -> TileInfo
    {
        let texture = texture_manager.get_texture(texture_path);
        TileInfo {
            id,
            texture,
            is_solid
        }
    }
}