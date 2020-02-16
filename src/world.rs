extern crate rand;

use rand::Rng;

const WORLD_SIZE: usize = 5;

pub struct World {
    nodes: [[i32; WORLD_SIZE]; WORLD_SIZE]
}

impl World {
    pub fn new() -> World {
        World{
            nodes: [[0;WORLD_SIZE];WORLD_SIZE]
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let random_number = rng.gen_range(0, 10);
                self.nodes[x][y] = random_number;     
            }
        }
    }

    pub fn log_world(&self)
    {
        for (i, row) in self.nodes.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                println!("[row={}][col={}]={}", i, j, col);
            }
        }
    }
}
