use macroquad::audio::{load_sound, play_sound, stop_sound, Sound, PlaySoundParams};
use crate::Rocket;
use macroquad::prelude::*;

pub struct Sounds {
    pub motor_hum: Sound,
    pub engine_fire: Sound,
    pub music: Sound,
    motor_playing: bool,
    engine_playing: bool,
}

impl Sounds {
    pub async fn load() -> Self {
        Self {
            motor_hum: load_sound("assets/motor_hum.ogg").await.unwrap(),
            engine_fire: load_sound("assets/engine_fire.ogg").await.unwrap(),
            music: load_sound("assets/music.ogg").await.unwrap(),
            motor_playing: false,
            engine_playing: false,
        }
    }

    pub fn start_music(&self) {
        play_sound(&self.music, PlaySoundParams { looped: true, volume: 0.5 });
    }

    pub fn update(&mut self, rocket: &Rocket) {
        let rotating = is_key_down(KeyCode::A) || is_key_down(KeyCode::D);
        if rotating && !self.motor_playing {
            play_sound(&self.motor_hum, PlaySoundParams { looped: true, volume: 0.15 });
            self.motor_playing = true;
        } else if !rotating && self.motor_playing {
            stop_sound(&self.motor_hum);
            self.motor_playing = false;
        }

        if rocket.engine_on && !self.engine_playing {
            play_sound(&self.engine_fire, PlaySoundParams { looped: true, volume: 0.3 });
            self.engine_playing = true;
        } else if !rocket.engine_on && self.engine_playing {
            stop_sound(&self.engine_fire);
            self.engine_playing = false;
        }
    }
}
