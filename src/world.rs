extern crate rand;

use crate::texture_manager::TextureManager;
use crate::tile::TileInfo;
use rand::Rng;
use sdl2::render::Texture;

const WORLD_SIZE: usize = 30;

pub struct World<'a> {
    nodes: [[u16; WORLD_SIZE]; WORLD_SIZE],
    tile_data: [TileInfo<'a>; 2]
}

impl World<'_> {
    pub fn new(texture_manager: &TextureManager) -> World {
        let tile_data: [TileInfo<'_>; 2] = [ 
            TileInfo::new(0, String::from("assets/tilemap/1.bmp"), false, texture_manager),
            TileInfo::new(1, String::from("assets/tilemap/2.bmp"), false, texture_manager) 
        ];

        World{
            nodes: [[0;WORLD_SIZE];WORLD_SIZE],
            tile_data
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut fill_counter = 0;
        let total_filled_nodes = (WORLD_SIZE * WORLD_SIZE) / 2;
        println!("NODES: {}", total_filled_nodes);
        while fill_counter < total_filled_nodes
        {
            let random_x = rng.gen_range(0, WORLD_SIZE);
            let random_y = rng.gen_range(0, WORLD_SIZE);
            if self.nodes[random_x][random_y] != 1
            {
                self.nodes[random_x][random_y] = 1;
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
        for (_y, row) in self.nodes.iter().enumerate() {
            println!("");
            for (_x, col) in row.iter().enumerate() {
                print!("{}", col);
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u16
    {
        self.nodes[x][y]
    }

    pub fn get_texture(&self, tile_id: u16) -> &Texture<'_>
    {
        &self.tile_data[tile_id as usize].texture
    }
}
