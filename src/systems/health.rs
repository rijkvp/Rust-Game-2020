use crate::components::Health;
use crate::resources::{play_die_sound, Sounds};
use amethyst::ecs::{Entities, Join, Read, ReadExpect, System, WriteStorage};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
};
use std::ops::Deref;

pub struct HealthSystem;

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        WriteStorage<'s, Health>,
        Entities<'s>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut healths, entities, asset_storage, sounds, audio_output): Self::SystemData,
    ) {
        for (e, health) in (&*entities, &mut healths).join() {
            if health.hp <= 0.0 {
                match entities.delete(e) {
                    Err(e) => {
                        panic!(e);
                    }
                    Ok(_t) => {}
                };
                play_die_sound(
                    &*sounds,
                    &asset_storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );
            }
        }
    }
}
