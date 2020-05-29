use crate::components::Health;
use amethyst::ecs::{Entities, Join, System, WriteStorage};

pub struct HealthSystem;

impl<'s> System<'s> for HealthSystem {
    type SystemData = (WriteStorage<'s, Health>, Entities<'s>);

    fn run(&mut self, (mut healths, entities): Self::SystemData) {
        for (e, health) in (&*entities, &mut healths).join() {
            if health.hp <= 0.0 {
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
