use sdl2::rect::Point;
use crate::world::World;
use crate::camera::Camera;
use crate::vectors::Vector2;
use sdl2::render::Texture;
use crate::texture_manager::TextureManager;

const TILE_SIZE: f32 = 64.0;

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

pub fn tile_to_world_coords(tile_x: u16, tile_y: u16, world: &World) -> Vector2
{
    Vector2 { 
        x: ((tile_x as f32) * TILE_SIZE - 0.5 * world.get_size() as f32 * TILE_SIZE),
        y: ((tile_y as f32) * TILE_SIZE - 0.5 * world.get_size() as f32 * TILE_SIZE),
    }
}

pub fn world_to_tile_coords(world_coords: Vector2, world: &World) -> Vector2
{
    Vector2 { 
        x: world_coords.x / TILE_SIZE + 0.5 * world.get_size() as f32,
        y: world_coords.y / TILE_SIZE + 0.5 * world.get_size() as f32 * TILE_SIZE,
    }
}

pub fn tile_to_screen_coords(tile_x: u16, tile_y: u16, camera: &Camera, world: &World) -> Point
{
    camera.world_to_screen(tile_to_world_coords(tile_x, tile_y, world))
}