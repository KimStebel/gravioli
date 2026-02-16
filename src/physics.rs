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

pub fn move_rocket(rocket: &mut Rocket, dt: f32) {
    rocket.x += rocket.speed_x * dt;
    rocket.y += rocket.speed_y * dt;
}

#[cfg(test)]
mod tests {
    use crate::Rocket;
    use super::*;

    fn make_rocket(x: f32, y: f32, speed_x: f32, speed_y: f32) -> Rocket {
        Rocket { x, y, speed_x, speed_y, orientation: 0.0, landed: false }
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
}
