use macroquad::prelude::*;

pub struct Planet {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct Rocket {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub orientation: f32, // degrees, 0/360 = up
    pub landed: bool,
    pub engine_on: bool,
    pub fuel: f32, // seconds of engine burn remaining
}

impl Rocket {
    fn initial() -> Self {
        Self {
            x: 100.0,
            y: screen_height() - 100.0,
            speed_x: 120.0,
            speed_y: 0.0,
            orientation: 90.0,
            landed: false,
            engine_on: false,
            fuel: 20.0,
        }
    }
}

pub struct GameState {
    pub planets: Vec<Planet>,
    pub rocket: Rocket,
    pub show_hud: bool,
    pub start_time: f64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            planets: vec![Planet {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
                radius: 30.0,
            }],
            rocket: Rocket::initial(),
            show_hud: true,
            start_time: get_time(),
        }
    }

    pub fn reset_rocket(&mut self) {
        self.rocket = Rocket::initial();
    }

    pub fn elapsed(&self) -> f64 {
        get_time() - self.start_time
    }
}
