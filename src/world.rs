extern crate rand;

use crate::texture_manager::TextureManager;
use crate::tile::TileInfo;
use rand::Rng;
use sdl2::render::Texture;

const DUNGEON_SIZE: usize = 5;
const TILES_PER_DUNGEON: usize = 10;
const WORLD_SIZE: usize = DUNGEON_SIZE * TILES_PER_DUNGEON;

pub struct World<'a> {
    dungeon_map: [[u16; DUNGEON_SIZE]; DUNGEON_SIZE],
    tile_map: [[u16; WORLD_SIZE]; WORLD_SIZE],
    tile_data: [TileInfo<'a>; 2]
}

impl World<'_> {
    pub fn new(texture_manager: &TextureManager) -> World {
        let tile_data: [TileInfo<'_>; 2] = [ 
            TileInfo::new(0, String::from("assets/tilemap/1.bmp"), false, texture_manager),
            TileInfo::new(1, String::from("assets/tilemap/2.bmp"), false, texture_manager) 
        ];

        World{
            dungeon_map: [[0;DUNGEON_SIZE];DUNGEON_SIZE],
            tile_map: [[0;WORLD_SIZE];WORLD_SIZE],
            tile_data
        }
    }

    pub fn generate(&mut self) {
        // Dungeon map
        let mut rng = rand::thread_rng();
        for x in 0..DUNGEON_SIZE
        {
            for y in 0..DUNGEON_SIZE
            {
                self.dungeon_map[x][y] = rng.gen_range(0, 2);
            }
        }

        // Tile map
        let mut fill_counter = 0;
        let total_filled_nodes = (WORLD_SIZE * WORLD_SIZE) / 2;
        println!("NODES: {}", total_filled_nodes);
        while fill_counter < total_filled_nodes
        {
            let random_x = rng.gen_range(0, WORLD_SIZE);
            let random_y = rng.gen_range(0, WORLD_SIZE);
            if self.tile_map[random_x][random_y] != 1
            {
                self.tile_map[random_x][random_y] = 1;
                fill_counter += 1;
            }
        }
    }

    pub fn get_size(&self) -> usize
    {
        WORLD_SIZE
    }

    pub fn log_world(&self)
    {
        for (_y, row) in self.dungeon_map.iter().enumerate() {
            println!("");
            for (_x, col) in row.iter().enumerate() {
                print!("{}", col);
            }
        }

        for (_y, row) in self.tile_map.iter().enumerate() {
            println!("");
            for (_x, col) in row.iter().enumerate() {
                print!("{}", col);
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u16
    {
        self.tile_map[x][y]
    }

    pub fn get_texture(&self, tile_id: u16) -> &Texture<'_>
    {
        &self.tile_data[tile_id as usize].texture
    }
}
