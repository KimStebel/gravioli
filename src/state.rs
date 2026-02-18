use macroquad::prelude::*;

#[derive(Clone)]
pub struct Planet {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Clone)]
pub struct Orbit {
    pub radius: f32,
    pub speed: f32,         // radians per second
    pub initial_angle: f32, // radians
}

#[derive(Clone)]
pub struct PlanetDef {
    pub center_x: f32,      // static position, or orbit center
    pub center_y: f32,
    pub radius: f32,        // planet body radius
    pub orbit: Option<Orbit>,
}

impl PlanetDef {
    pub fn planet_at(&self, time: f64) -> Planet {
        match &self.orbit {
            None => Planet {
                x: self.center_x,
                y: self.center_y,
                radius: self.radius,
            },
            Some(orbit) => {
                let angle = orbit.initial_angle + orbit.speed * time as f32;
                Planet {
                    x: self.center_x + orbit.radius * angle.cos(),
                    y: self.center_y + orbit.radius * angle.sin(),
                    radius: self.radius,
                }
            }
        }
    }
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

#[derive(Clone)]
pub enum WinCondition {
    Circle { x: f32, y: f32, radius: f32, max_speed: f32 },
    CircleAnySpeed { x: f32, y: f32, radius: f32 },
}

impl WinCondition {
    pub fn description(&self) -> String {
        match self {
            WinCondition::Circle { max_speed, .. } => {
                format!("Reach the green circle at under {:.0} px/s with engine off", max_speed)
            }
            WinCondition::CircleAnySpeed { .. } => {
                "Reach the green circle with engine off".to_string()
            }
        }
    }
}

#[derive(Clone)]
pub struct Level {
    pub name: &'static str,
    pub planets: Vec<PlanetDef>,
    pub initial_rocket: Rocket,
    pub win_condition: WinCondition,
}

impl Level {
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                name: "Level 1",
                planets: vec![
                    PlanetDef {
                        center_x: screen_width() / 2.0,
                        center_y: screen_height() / 2.0,
                        radius: 30.0,
                        orbit: None,
                    },
                    PlanetDef {
                        center_x: screen_width() / 2.0,
                        center_y: screen_height() / 2.0,
                        radius: 15.0,
                        orbit: Some(Orbit {
                            radius: 150.0,
                            speed: 0.5,
                            initial_angle: 0.0,
                        }),
                    },
                ],
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
                win_condition: WinCondition::CircleAnySpeed {
                    x: screen_width() - 150.0,
                    y: 150.0,
                    radius: 50.0,
                },
            },
            Self {
                name: "Level 2",
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
                win_condition: WinCondition::Circle {
                    x: screen_width() - 150.0,
                    y: 150.0,
                    radius: 50.0,
                    max_speed: 2.0,
                },
            },
            // Two large planets guard a narrow gap; thread the needle to reach the goal
            Self {
                name: "The Gauntlet",
                planets: vec![
                    PlanetDef {
                        center_x: screen_width() / 2.0,
                        center_y: screen_height() / 2.0 - 120.0,
                        radius: 45.0,
                        orbit: None,
                    },
                    PlanetDef {
                        center_x: screen_width() / 2.0,
                        center_y: screen_height() / 2.0 + 120.0,
                        radius: 45.0,
                        orbit: None,
                    },
                    PlanetDef {
                        center_x: screen_width() * 0.8,
                        center_y: screen_height() / 2.0,
                        radius: 20.0,
                        orbit: None,
                    },
                ],
                initial_rocket: Rocket {
                    x: 100.0,
                    y: screen_height() / 2.0,
                    speed_x: 150.0,
                    speed_y: 0.0,
                    orientation: 90.0,
                    landed: false,
                    engine_on: false,
                    fuel: 8.0,
                },
                win_condition: WinCondition::CircleAnySpeed {
                    x: screen_width() - 100.0,
                    y: screen_height() / 2.0,
                    radius: 60.0,
                },
            },
            // Two planets locked in orbit around each other; navigate the chaos
            Self {
                name: "Binary Stars",
                planets: vec![
                    PlanetDef {
                        center_x: screen_width() / 2.0,
                        center_y: screen_height() / 2.0,
                        radius: 35.0,
                        orbit: Some(Orbit {
                            radius: 120.0,
                            speed: 0.8,
                            initial_angle: 0.0,
                        }),
                    },
                    PlanetDef {
                        center_x: screen_width() / 2.0,
                        center_y: screen_height() / 2.0,
                        radius: 35.0,
                        orbit: Some(Orbit {
                            radius: 120.0,
                            speed: 0.8,
                            initial_angle: std::f32::consts::PI,
                        }),
                    },
                ],
                initial_rocket: Rocket {
                    x: screen_width() / 2.0,
                    y: 80.0,
                    speed_x: 0.0,
                    speed_y: 0.0,
                    orientation: 180.0,
                    landed: false,
                    engine_on: false,
                    fuel: 15.0,
                },
                win_condition: WinCondition::Circle {
                    x: screen_width() / 2.0,
                    y: screen_height() - 80.0,
                    radius: 60.0,
                    max_speed: 50.0,
                },
            },
            // One massive planet with a fast moon; use gravity to slingshot with minimal fuel
            Self {
                name: "Slingshot",
                planets: vec![
                    PlanetDef {
                        center_x: screen_width() * 0.35,
                        center_y: screen_height() / 2.0,
                        radius: 50.0,
                        orbit: None,
                    },
                    PlanetDef {
                        center_x: screen_width() * 0.35,
                        center_y: screen_height() / 2.0,
                        radius: 12.0,
                        orbit: Some(Orbit {
                            radius: 180.0,
                            speed: -1.2,
                            initial_angle: 0.0,
                        }),
                    },
                ],
                initial_rocket: Rocket {
                    x: screen_width() - 120.0,
                    y: 100.0,
                    speed_x: -80.0,
                    speed_y: 30.0,
                    orientation: 270.0,
                    landed: false,
                    engine_on: false,
                    fuel: 4.0,
                },
                win_condition: WinCondition::CircleAnySpeed {
                    x: 120.0,
                    y: screen_height() - 120.0,
                    radius: 70.0,
                },
            },
        ]
    }
}

#[derive(Clone)]
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

    pub fn current_planets(&self) -> Vec<Planet> {
        let elapsed = self.elapsed();
        self.level.planets.iter().map(|p| p.planet_at(elapsed)).collect()
    }
}

#[derive(Clone)]
pub struct GameState {
    pub level: LevelState,
    pub show_hud: bool,
    pub show_path: bool,
}

impl GameState {
    pub fn new(level: Level) -> Self {
        Self {
            level: LevelState::new(level),
            show_hud: true,
            show_path: true,
        }
    }
}
