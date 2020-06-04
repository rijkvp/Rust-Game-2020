use amethyst::{
    assets::{Loader, AssetStorage},
    audio::{OggFormat, AudioSink, SourceHandle},
    ecs::{World, WorldExt},
    audio::{output::Output, Source},
};
use std::{iter::Cycle, vec::IntoIter};

const FIRE_SOUND: &str = "audio/fire.ogg";
const DAMAGE_SOUND: &str = "audio/damage.ogg";
const DIE_SOUND: &str = "audio/die.ogg";

const MUSIC_TRACKS: &[&str] = &[
    "audio/menu_music.ogg",
    "audio/game_music.ogg",
];

pub struct Sounds {
    pub fire_sfx: SourceHandle,
    pub damage_sfx: SourceHandle,
    pub die_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialise_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.05); 

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            fire_sfx: load_audio_track(&loader, &world, FIRE_SOUND),
            damage_sfx: load_audio_track(&loader, &world, DAMAGE_SOUND),
            die_sfx: load_audio_track(&loader, &world, DIE_SOUND),
        };
        (sound, music)
    };

    // Add sound effects and music to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}


pub fn play_fire_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.fire_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_damage_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.damage_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}


pub fn play_die_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.die_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

