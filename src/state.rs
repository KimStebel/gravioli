use macroquad::prelude::*;

pub struct Planet {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Clone)]
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

pub struct Level {
    pub planets: Vec<Planet>,
    pub initial_rocket: Rocket,
}

impl Level {
    pub fn one() -> Self {
        Self {
            planets: vec![Planet {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
                radius: 30.0,
            }],
            initial_rocket: Rocket {
                x: 100.0,
                y: screen_height() - 100.0,
                speed_x: 120.0,
                speed_y: 0.0,
                orientation: 90.0,
                landed: false,
                engine_on: false,
                fuel: 20.0,
            },
        }
    }

    pub fn two() -> Self {
        Self {
            planets: vec![],
            initial_rocket: Rocket {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
                speed_x: 0.0,
                speed_y: 0.0,
                orientation: 0.0,
                landed: false,
                engine_on: false,
                fuel: 20.0,
            },
        }
    }
}

pub struct LevelState {
    pub level: Level,
    pub rocket: Rocket,
    pub start_time: f64,
}

impl LevelState {
    pub fn new(design: Level) -> Self {
        let rocket = design.initial_rocket.clone();
        Self { level: design, rocket, start_time: get_time() }
    }

    pub fn reset_rocket(&mut self) {
        self.rocket = self.level.initial_rocket.clone();
    }

    pub fn elapsed(&self) -> f64 {
        get_time() - self.start_time
    }
}

pub struct GameState {
    pub level: LevelState,
    pub show_hud: bool,
}

impl GameState {
    pub fn new(level: Level) -> Self {
        Self {
            level: LevelState::new(level),
            show_hud: true,
        }
    }
}
