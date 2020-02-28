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

    pub fn log_world(&self)
    {
        for (_y, row) in self.nodes.iter().enumerate() {
            println!("");
            for (_x, col) in row.iter().enumerate() {
                print!("{}", col);
            }
        }
    }
}
