use macroquad::audio::{load_sound, play_sound, stop_sound, Sound, PlaySoundParams};
use crate::state::Rocket;
use macroquad::prelude::*;

pub struct Sounds {
    motor_hum: Sound,
    engine_fire: Sound,
    explosion: Sound,
    level_complete: Sound,
    music: Sound,
    music_volume: f32,
    effect_volume: f32,
    motor_playing: bool,
    engine_playing: bool,
}

impl Sounds {
    pub async fn load() -> Self {
        let mut storage = quad_storage::STORAGE.lock().unwrap();

        let music_volume = match storage.get("music_volume") {
            Some(v) => v.parse::<f32>().unwrap_or(0.5),
            None => {
                storage.set("music_volume", "0.5");
                0.5
            }
        };

        let effect_volume = match storage.get("effect_volume") {
            Some(v) => v.parse::<f32>().unwrap_or(1.0),
            None => {
                storage.set("effect_volume", "1");
                1.0
            }
        };

        drop(storage);

        let sfx = "assets/sounds";
        Self {
            motor_hum: load_sound(&format!("{sfx}/motor_hum.ogg")).await.unwrap(),
            engine_fire: load_sound(&format!("{sfx}/engine_fire.ogg")).await.unwrap(),
            explosion: load_sound(&format!("{sfx}/explosion.ogg")).await.unwrap(),
            level_complete: load_sound(&format!("{sfx}/level_complete.aif")).await.unwrap(),
            music: load_sound("assets/music.ogg").await.unwrap(),
            music_volume,
            effect_volume,
            motor_playing: false,
            engine_playing: false,
        }
    }

    pub fn start_music(&self) {
        play_sound(&self.music, PlaySoundParams { looped: true, volume: self.music_volume });
    }

    pub fn play_level_complete(&self) {
        play_sound(&self.level_complete, PlaySoundParams { looped: false, volume: self.effect_volume });
    }

    pub fn play_explosion(&self) {
        play_sound(&self.explosion, PlaySoundParams { looped: false, volume: self.effect_volume });
    }

    pub fn update(&mut self, rocket: &Rocket) {
        let rotating = is_key_down(KeyCode::A) || is_key_down(KeyCode::D);
        if rotating && !self.motor_playing {
            play_sound(&self.motor_hum, PlaySoundParams { looped: true, volume: 0.15 * self.effect_volume });
            self.motor_playing = true;
        } else if !rotating && self.motor_playing {
            stop_sound(&self.motor_hum);
            self.motor_playing = false;
        }

        if rocket.engine_on && !self.engine_playing {
            play_sound(&self.engine_fire, PlaySoundParams { looped: true, volume: 0.3 * self.effect_volume });
            self.engine_playing = true;
        } else if !rocket.engine_on && self.engine_playing {
            stop_sound(&self.engine_fire);
            self.engine_playing = false;
        }
    }
}
