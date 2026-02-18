use macroquad::prelude::*;

mod controls;
mod drawing;
mod images;
mod menu;
mod physics;
mod sound;
mod state;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gravioli".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[derive(Clone)]
enum Screen {
    Menu,
    Playing(state::GameState),
}

#[macroquad::main(window_conf)]
async fn main() {
    info!("Starting Gravioli");
    let images = images::Images::load().await;
    let mut sounds = sound::Sounds::load().await;
    sounds.start_music();
    // these two are needed because the first frames are too long due to the asset loading
    next_frame().await;
    next_frame().await;
    let levels = state::Level::all();
    let mut menu = menu::Menu::new(&levels);
    let mut screen = Screen::Menu;

    loop {
        match &mut screen {
            Screen::Menu => {
                match menu.update() {
                    Some(menu::MenuChoice::Play(i)) if i < levels.len() => {
                        screen = Screen::Playing(state::GameState::new(levels[i].clone()));
                    }
                    Some(menu::MenuChoice::Exit) => return,
                    _ => {}
                }
            }
            Screen::Playing(game) => {
                let dt = get_frame_time();
                let mut return_to_menu = false;
                if controls::handle_input(&mut game.level.rocket, dt, &mut game.show_hud) {
                    return_to_menu = true;
                } else {
                    sounds.update(&game.level.rocket);
                    match physics::update(game, dt) {
                        Some(physics::PhysicsEvent::Collision) => {
                            sounds.play_explosion();
                        }
                        Some(physics::PhysicsEvent::Win) => {
                            sounds.play_level_complete();
                            return_to_menu = true;
                        }
                        None => {}
                    }
                    drawing::draw(&game.level.level.planets, &game.level.rocket, &game.level.level.win_condition, &images, game.level.elapsed(), game.show_hud);
                }
                if return_to_menu {
                    screen = Screen::Menu;
                }
            }
        }
        next_frame().await;
    }
}
