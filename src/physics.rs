use macroquad::prelude::*;
use crate::{Planet, Rocket};

pub fn update_rocket_speed(rocket: &mut Rocket, planet: &Planet) {
    let dt = get_frame_time();
    let dx = planet.x - rocket.x;
    let dy = planet.y - rocket.y;
    let dist_sq = dx * dx + dy * dy;
    let dist = dist_sq.sqrt();
    let gravity = 4000000.0;
    let accel = gravity / dist_sq;
    rocket.speed_x += (dx / dist) * accel * dt;
    rocket.speed_y += (dy / dist) * accel * dt;
}

pub fn move_rocket(rocket: &mut Rocket) {
    let dt = get_frame_time();
    rocket.x += rocket.speed_x * dt;
    rocket.y += rocket.speed_y * dt;
}
