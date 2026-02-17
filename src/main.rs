use macroquad::prelude::*;

mod controls;
mod drawing;
mod images;
mod physics;
mod sound;
mod state;

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
    info!("Starting Garlic");
    let images = images::Images::load().await;
    let mut sounds = sound::Sounds::load().await;
    sounds.start_music();
    // these two are needed because the first frames are too long due to the asset loading
    next_frame().await;
    next_frame().await;
    let mut game = state::GameState::new();

    loop {
        let dt = get_frame_time();
        controls::handle_input(&mut game.level.rocket, dt, &mut game.show_hud);
        sounds.update(&game.level.rocket);
        physics::update(&mut game, dt);
        drawing::draw(&game.level.level.planets, &game.level.rocket, &images, game.level.elapsed(), game.show_hud);
        next_frame().await;
    }
}
