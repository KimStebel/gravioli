use macroquad::prelude::*;
use crate::{Planet, Rocket};

pub fn draw_planet(planet: &Planet) {
    draw_circle(planet.x, planet.y, planet.radius, Color::new(0.76, 0.60, 0.42, 1.0));
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
}

pub fn draw_hud(elapsed: f64) {
    draw_text(&format!("{:.1}s", elapsed), screen_width() - 80.0, screen_height() - 20.0, 24.0, WHITE);
}
