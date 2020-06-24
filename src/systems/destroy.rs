use crate::components::Damageable;
use amethyst::ecs::{Entities, Join, System, ReadStorage};

pub struct DestroySystem;

impl<'s> System<'s> for DestroySystem {
    type SystemData = (ReadStorage<'s, Damageable>, Entities<'s>);

    fn run(&mut self, (damageables, entities): Self::SystemData) {
        for (e, damageable) in (&*entities, &damageables).join() {
            if damageable.destroyed {
                match entities.delete(e) {
                    Err(e) => {
                        panic!(e);
                    }
                    Ok(_t) => {}
                };
            }
        }
    }
}
