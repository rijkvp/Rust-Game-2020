use crate::components::{Lifetime};
use amethyst::core::Time;
use amethyst::ecs::{Join, Read, System, WriteStorage, Entities};

pub struct LifetimeSystem;

impl<'s> System<'s> for LifetimeSystem
{
    type SystemData = (WriteStorage<'s, Lifetime>, Read<'s, Time>, Entities<'s>);

    fn run(&mut self, (mut lifetimes, time, entities): Self::SystemData) { 
        for (e, lt) in (&*entities, &mut lifetimes).join()
        {
            lt.lifetime -= time.delta_seconds();
            if lt.lifetime <= 0.0
            {
                match entities.delete(e) {
                    Err(e) => {
                        panic!(e);
                    },
                    Ok(_t) => {}
                };
            }
        }
        // println!("FPS: {} ", ((1.0 / time.delta_seconds()) as i32));
    }
}