use macroquad::prelude::*;
use crate::state::{Planet, Rocket};

pub fn update_rocket_speed(rocket: &mut Rocket, planet: &Planet, dt: f32) {
    let dx = planet.x - rocket.x;
    let dy = planet.y - rocket.y;
    let dist_sq = dx * dx + dy * dy;
    let dist = dist_sq.sqrt();
    let gravity = 4000000.0;
    let accel = gravity / dist_sq;
    rocket.speed_x += (dx / dist) * accel * dt;
    rocket.speed_y += (dy / dist) * accel * dt;

    if rocket.engine_on {
        apply_thrust(rocket, dt);
    }
}

pub fn apply_thrust(rocket: &mut Rocket, dt: f32) {
    let thrust = 10.0; // pixels per second squared
    let angle = rocket.orientation.to_radians();
    rocket.speed_x += angle.sin() * thrust * dt;
    rocket.speed_y -= angle.cos() * thrust * dt;
}

pub fn move_rocket(rocket: &mut Rocket, dt: f32) {
    rocket.x += rocket.speed_x * dt;
    rocket.y += rocket.speed_y * dt;
}

pub fn check_collision(rocket: &Rocket, planet: &Planet) -> bool {
    let dx = rocket.x - planet.x;
    let dy = rocket.y - planet.y;
    dx * dx + dy * dy < planet.radius * planet.radius
}

#[cfg(test)]
mod tests {
    use crate::state::Rocket;
    use super::*;

    fn make_rocket(x: f32, y: f32, speed_x: f32, speed_y: f32) -> Rocket {
        Rocket { x, y, speed_x, speed_y, orientation: 0.0, landed: false, engine_on: false }
    }

    #[test]
    fn moves_right() {
        let mut rocket = make_rocket(0.0, 0.0, 100.0, 0.0);
        move_rocket(&mut rocket, 1.0);
        assert_eq!(rocket.x, 100.0);
        assert_eq!(rocket.y, 0.0);
    }

    #[test]
    fn moves_down() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 50.0);
        move_rocket(&mut rocket, 2.0);
        assert_eq!(rocket.x, 0.0);
        assert_eq!(rocket.y, 100.0);
    }

    #[test]
    fn moves_diagonally() {
        let mut rocket = make_rocket(10.0, 20.0, 30.0, -40.0);
        move_rocket(&mut rocket, 0.5);
        assert_eq!(rocket.x, 25.0);
        assert_eq!(rocket.y, 0.0);
    }

    #[test]
    fn stationary_with_zero_speed() {
        let mut rocket = make_rocket(5.0, 10.0, 0.0, 0.0);
        move_rocket(&mut rocket, 1.0);
        assert_eq!(rocket.x, 5.0);
        assert_eq!(rocket.y, 10.0);
    }

    #[test]
    fn zero_dt_no_movement() {
        let mut rocket = make_rocket(5.0, 10.0, 100.0, 200.0);
        move_rocket(&mut rocket, 0.0);
        assert_eq!(rocket.x, 5.0);
        assert_eq!(rocket.y, 10.0);
    }

    fn make_planet(x: f32, y: f32) -> Planet {
        Planet { x, y, radius: 30.0 }
    }

    #[test]
    fn accelerates_toward_planet_on_right() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        let planet = make_planet(100.0, 0.0);
        update_rocket_speed(&mut rocket, &planet, 1.0);
        assert!(rocket.speed_x > 0.0);
        assert_eq!(rocket.speed_y, 0.0);
    }

    #[test]
    fn accelerates_toward_planet_above() {
        let mut rocket = make_rocket(0.0, 100.0, 0.0, 0.0);
        let planet = make_planet(0.0, 0.0);
        update_rocket_speed(&mut rocket, &planet, 1.0);
        assert_eq!(rocket.speed_x, 0.0);
        assert!(rocket.speed_y < 0.0);
    }

    #[test]
    fn stronger_gravity_when_closer() {
        let mut close = make_rocket(0.0, 0.0, 0.0, 0.0);
        let mut far = make_rocket(0.0, 0.0, 0.0, 0.0);
        let close_planet = make_planet(100.0, 0.0);
        let far_planet = make_planet(1000.0, 0.0);
        update_rocket_speed(&mut close, &close_planet, 1.0);
        update_rocket_speed(&mut far, &far_planet, 1.0);
        assert!(close.speed_x > far.speed_x);
    }

    #[test]
    fn zero_dt_no_acceleration() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        let planet = make_planet(100.0, 0.0);
        update_rocket_speed(&mut rocket, &planet, 0.0);
        assert_eq!(rocket.speed_x, 0.0);
        assert_eq!(rocket.speed_y, 0.0);
    }

    #[test]
    fn accelerates_diagonally() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        update_rocket_speed(&mut rocket, &planet, 1.0);
        assert!(rocket.speed_x > 0.0);
        assert!(rocket.speed_y > 0.0);
        assert!((rocket.speed_x - rocket.speed_y).abs() < f32::EPSILON);
    }
}
