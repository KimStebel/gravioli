use macroquad::prelude::*;
use crate::{Planet, Rocket};

pub fn draw_planet(planet: &Planet, texture: &Texture2D) {
    let size = planet.radius * 2.0;
    draw_texture_ex(texture, planet.x - planet.radius, planet.y - planet.radius, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(size, size)),
        ..Default::default()
    });
}

pub fn draw_rocket(rocket: &Rocket) {
    let body_width = 10.0;
    let body_height = 30.0;
    let nose_height = 10.0;

    let angle = rocket.orientation.to_radians();
    let rotate = |lx: f32, ly: f32| -> (f32, f32) {
        let rx = lx * angle.cos() - ly * angle.sin();
        let ry = lx * angle.sin() + ly * angle.cos();
        (rocket.x + rx, rocket.y + ry)
    };

    // Body rectangle (two triangles)
    let bl = rotate(-body_width / 2.0, 0.0);
    let br = rotate(body_width / 2.0, 0.0);
    let tl = rotate(-body_width / 2.0, -body_height);
    let tr = rotate(body_width / 2.0, -body_height);

    draw_triangle(bl.into(), br.into(), tr.into(), WHITE);
    draw_triangle(bl.into(), tr.into(), tl.into(), WHITE);

    // Nose cone
    let tip = rotate(0.0, -body_height - nose_height);
    draw_triangle(tl.into(), tr.into(), tip.into(), WHITE);

    // Engine flames
    if rocket.engine_on {
        let flame_height = 15.0;
        let fl = rotate(-body_width / 3.0, 0.0);
        let fr = rotate(body_width / 3.0, 0.0);
        let ftip = rotate(0.0, flame_height);
        draw_triangle(fl.into(), fr.into(), ftip.into(), ORANGE);
    }
}

pub fn draw_hud(elapsed: f64, rocket: &Rocket) {
    let speed = (rocket.speed_x * rocket.speed_x + rocket.speed_y * rocket.speed_y).sqrt();
    draw_text(&format!("Time: {:.1}s", elapsed), screen_width() - 200.0, screen_height() - 40.0, 24.0, WHITE);
    draw_text(&format!("Speed: {:.0} px/s", speed), screen_width() - 200.0, screen_height() - 20.0, 24.0, WHITE);
}
