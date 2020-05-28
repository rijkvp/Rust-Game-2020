use crate::components::{Damageable, Health, Physics, PhysicsLayer, PhysicsType};
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::vectors::Vector2;

struct AABB {
    pub do_collision: bool,
    pub deal_damage: bool,
    pub damage: f32,
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl PartialEq for AABB {
    fn eq(&self, other: &Self) -> bool {
        self.x1 == other.x1 && self.x2 == other.x2 && self.y1 == other.y1 && self.y2 == other.y2
    }
}

impl AABB {
    pub fn from_center(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        do_collision: bool,
        deal_damage: bool,
        damage: f32,
    ) -> AABB {
        AABB {
            x1: x - width / 2.0,
            x2: x + width / 2.0,
            y1: y - height / 2.0,
            y2: y + height / 2.0,
            do_collision,
            deal_damage,
            damage,
        }
    }

    pub fn get_points(&self) -> Vec<Vector2> {
        let mut points = Vec::new();
        points.push(Vector2 {
            x: self.x1,
            y: self.y1,
        });
        points.push(Vector2 {
            x: self.x2,
            y: self.y1,
        });
        points.push(Vector2 {
            x: self.x1,
            y: self.y2,
        });
        points.push(Vector2 {
            x: self.x2,
            y: self.y2,
        });
        return points;
    }
}

pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Physics>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Damageable>,
        WriteStorage<'s, Health>,
    );

    fn run(&mut self, (mut physics, mut transforms, damageables, mut healths): Self::SystemData) {
        const SCALE_MULTIPLIER: f32 = 50.0;
        let mut colliders = Vec::<AABB>::new();
        // TEMP FIX
        // TODO: Add colliders in proper way
        // for (transf, phys) in (&transforms, &physics).join() {
        //     colliders.push(AABB::from_center(
        //         transf.translation().x,
        //         transf.translation().y,
        //         transf.scale().x * SCALE_MULTIPLIER,
        //         transf.scale().y * SCALE_MULTIPLIER,
        //         PhysicsLayer::collidable(phys.layer),
        //         false,
        //         0.0,
        //     ));
        // }

        for (transf, phys, damageable) in (&transforms, &physics, &damageables).join() {
            let aabb = AABB::from_center(
                transf.translation().x,
                transf.translation().y,
                transf.scale().x * SCALE_MULTIPLIER,
                transf.scale().y * SCALE_MULTIPLIER,
                PhysicsLayer::collidable(phys.layer),
                true,
                damageable.damage,
            );
            colliders.push(aabb);
        }

        for (phys, transf) in (&mut physics, &mut transforms).join() {
            match phys.physics_type {
                PhysicsType::Static => {}
                PhysicsType::Dynamic => {
                    let collider1 = AABB::from_center(
                        transf.translation().x,
                        transf.translation().y,
                        transf.scale().x * SCALE_MULTIPLIER,
                        transf.scale().y * SCALE_MULTIPLIER,
                        PhysicsLayer::collidable(phys.layer),
                        false,
                        0.0,
                    );
                    let mut did_collide = false;
                    for collider2 in colliders.iter() {
                        if collider1 == *collider2 {
                            continue;
                        }
                        for point in collider2.get_points().iter() {
                            if point.x >= collider1.x1
                                && point.x <= collider1.x2
                                && point.y >= collider1.y1
                                && point.y <= collider1.y2
                            {
                                did_collide = collider1.do_collision && collider2.do_collision;
                                if collider2.deal_damage {
                                    println!(
                                        "DEAL {} DAMAGE FROM {} TO {}",
                                        collider2.damage, collider1.x1, collider2.x2
                                    );
                                }
                            }
                        }
                    }
                    const DRAG: f32 = 10.0;
                    const DELTA_MULTIPLIER: f32 = 1.0 / 60.0;
                    if did_collide {
                        phys.velocity = Vector2::default();
                    }
                    if phys.drag {
                        phys.velocity = phys.velocity * (1.0 - DELTA_MULTIPLIER * DRAG);
                    }
                    transf.append_translation((phys.velocity * DELTA_MULTIPLIER).to_vector3());
                }
            }
        }
    }
}
