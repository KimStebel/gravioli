use macroquad::prelude::*;

mod controls;
mod drawing;
mod images;
mod physics;
mod sound;

pub struct Planet {
    pub x: f32,
    pub y: f32,
    pub radius: f32
}
impl Planet {

}

pub struct Rocket {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub orientation: f32, // degrees, 0/360 = up
    pub landed: bool,
    pub engine_on: bool,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Garlic".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let planet = Planet {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        radius: 30.0
    };

    let mut rocket = Rocket {
        x: 100.0,
        y: screen_height() - 100.0,
        speed_x: 200.0,
        speed_y: 0.0,
        orientation: 90.0,
        landed: false,
        engine_on: false,
    };

    let images = images::Images::load().await;
    let mut sounds = sound::Sounds::load().await;
    sounds.start_music();

    let start_time = get_time();
    let mut boosted = false;
    let mut show_hud = true;

    next_frame().await;
    next_frame().await;

    loop {
        let elapsed = get_time() - start_time;
        if !boosted && elapsed >= 5.0 {
            rocket.speed_x -= 120.0;
            rocket.speed_y += 60.0;
            boosted = true;
        }
        let dt = get_frame_time();
        controls::handle_input(&mut rocket, dt, &mut show_hud);
        sounds.update(&rocket);
        physics::update_rocket_speed(&mut rocket, &planet, dt);
        physics::move_rocket(&mut rocket, dt);
        drawing::draw(&planet, &rocket, &images, elapsed, show_hud);
        next_frame().await;
    }
}
