extern crate rand;

use crate::bullet::BulletsManager;
use crate::player::Player;
use crate::physics::PhysicsManager;
use crate::enemy::Enemy;
use crate::enemy::EnemyType;
use crate::vectors::Vector2;
use crate::texture_manager::TextureManager;
use crate::tile::*;
use rand::Rng;
use sdl2::render::Texture;

const DUNGEON_SIZE: usize = 5;
const TILES_PER_DUNGEON: usize = 10;
const WORLD_SIZE: usize = DUNGEON_SIZE * TILES_PER_DUNGEON;
const TILESET_SIZE: usize = 3;

pub struct World<'a> {
    dungeon_map: [[u16; DUNGEON_SIZE]; DUNGEON_SIZE],
    tile_map: [[u16; WORLD_SIZE]; WORLD_SIZE],
    tile_data: [TileInfo<'a>; TILESET_SIZE],
    pub enemies: Vec::<Enemy>,
}

impl<'a> World<'a> {
    pub fn new(texture_manager: &'a TextureManager) -> World<'a> {
        let tile_data: [TileInfo<'_>; TILESET_SIZE] = [ 
            TileInfo::new(0, String::from("assets/tilemap/1.bmp"), true, texture_manager),
            TileInfo::new(1, String::from("assets/tilemap/2.bmp"), false, texture_manager),
            TileInfo::new(2, String::from("assets/tilemap/3.bmp"), false, texture_manager) 
        ];

        World {
            dungeon_map: [[0;DUNGEON_SIZE];DUNGEON_SIZE],
            tile_map: [[0;WORLD_SIZE];WORLD_SIZE],
            tile_data,
            enemies: Vec::<Enemy>::new(),
        }
    }

    pub fn generate(&mut self, pm: &mut PhysicsManager) {
        let mut rng = rand::thread_rng();
        // Dungeon map
        // TODO: Generate dungeon map like this:
        // let mut rng = rand::thread_rng();
        // for x in 0..DUNGEON_SIZE
        // {
        //     for y in 0..DUNGEON_SIZE
        //     {
        //         self.dungeon_map[x][y] = rng.gen_range(0, 2);
        //     }
        // }

        // Temp fix
        self.dungeon_map = [ 
            [0,0,0,0,0],
            [0,0,2,0,0],
            [0,0,1,0,0],
            [0,1,0,0,0],
            [1,0,0,0,0],
        ];


        // Tile map
        // let mut fill_counter = 0;
        // let total_filled_nodes = (WORLD_SIZE * WORLD_SIZE) / 2;
        // println!("NODES: {}", total_filled_nodes);
        // while fill_counter < total_filled_nodes
        // {
        //     let random_x = rng.gen_range(0, WORLD_SIZE);
        //     let random_y = rng.gen_range(0, WORLD_SIZE);
        //     if self.tile_map[random_x][random_y] != 1
        //     {
        //         self.tile_map[random_x][random_y] = 1;
        //         fill_counter += 1;
        //     }
        // }
        for dungeon_x in 0..DUNGEON_SIZE
        {
            for dungeon_y in 0..DUNGEON_SIZE
            {
                let dungeon_value = self.dungeon_map[dungeon_x][dungeon_y];
                let start_x = dungeon_x * TILES_PER_DUNGEON;
                let start_y = dungeon_y * TILES_PER_DUNGEON;
                let end_x = dungeon_x * TILES_PER_DUNGEON + TILES_PER_DUNGEON;
                let end_y = dungeon_y * TILES_PER_DUNGEON + TILES_PER_DUNGEON;
                
                if dungeon_value == 2
                {
                    let mut spawnpoints = Vec::<Vector2>::new();
                    spawnpoints.push(tile_to_world_coords((start_x + 1) as u16, (start_y + 1) as u16, &self));
                    spawnpoints.push(tile_to_world_coords((end_x - 2) as u16, (start_y + 1) as u16, &self));
                    spawnpoints.push(tile_to_world_coords((start_x + 1) as u16, (end_y - 2) as u16, &self));
                    spawnpoints.push(tile_to_world_coords((end_x - 2) as u16, (end_y - 2) as u16, &self));
                    spawnpoints.push(tile_to_world_coords((start_x + (end_x - start_x) / 2) as u16, (start_y + (end_y - start_y) / 2) as u16, &self));

                    for spawnpoint in spawnpoints.iter()
                    {
                        let enemy_type = match rng.gen_range(0, 2)
                        {
                            0 => EnemyType::Melee,
                            1 => EnemyType::Range,
                            _ => panic!(),
                        };
                        self.enemies.push(
                            Enemy::new(
                                Vector2 { x: spawnpoint.x, y: spawnpoint.y },
                                pm,
                                enemy_type,
                            )
                        );
                    }
                }

                for x in start_x..end_x
                {
                    for y in start_y..end_y
                    {
                        self.tile_map[x][y] = dungeon_value;
                        
                    }
                }
            }
        }
    }

    pub fn update_enemies(&mut self, player: &mut Player, pm: &mut PhysicsManager, bullets_manager: &mut BulletsManager, delta_time: f32)
    {
        for enemy in self.enemies.iter_mut() {
            enemy.update(player, pm, bullets_manager, delta_time);
        }
        self.enemies.retain(|enemy| !enemy.is_dead);
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

    pub fn set_surrounding(&mut self, position: Vector2)
    {
        let x: usize = position.x as usize;
        let y: usize = position.y as usize;

        self.tile_map[x + 1][y] = 2;
        self.tile_map[x][y + 1] = 2;
        self.tile_map[x + 1][y + 1] = 2;
        self.tile_map[x][y] = 2;
        self.tile_map[x - 1][y] = 2;
        self.tile_map[x][y - 1] = 2;
        self.tile_map[x - 1][y - 1] = 2;
        self.tile_map[x - 1][y + 1] = 2;
        self.tile_map[x + 1][y - 1] = 2;
    }

    pub fn get_surrounding(&mut self, position: Vector2) -> [[u16; 3]; 3]
    {
        let x_pos: usize = position.x as usize;
        let y_pos: usize = position.y as usize;

        let mut surrounding: [[u16; 3]; 3] = [[0; 3]; 3];

        for x in 0..3
        {
            for y in 0..3
            {
               surrounding[x][y] = self.tile_map[x_pos -1 + x][y_pos -1 + y];
            }
        }
        return surrounding;
    }
}
