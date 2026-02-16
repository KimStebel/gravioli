use macroquad::prelude::*;
use crate::Rocket;

pub fn handle_input(rocket: &mut Rocket, dt: f32, show_hud: &mut bool) {
    update_orientation(rocket, dt, is_key_down(KeyCode::A), is_key_down(KeyCode::D));
    if is_key_pressed(KeyCode::Z) {
        rocket.engine_on = true;
    }
    if is_key_pressed(KeyCode::X) {
        rocket.engine_on = false;
    }
    if is_key_pressed(KeyCode::H) {
        *show_hud = !*show_hud;
    }
}

pub fn update_orientation(rocket: &mut Rocket, dt: f32, rotate_left: bool, rotate_right: bool) {
    let rotation_speed = 360.0 / 4.0; // degrees per second
    if rotate_right {
        rocket.orientation += rotation_speed * dt;
    }
    if rotate_left {
        rocket.orientation -= rotation_speed * dt;
    }
    rocket.orientation = rocket.orientation.rem_euclid(360.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rocket;

    fn make_rocket(orientation: f32) -> Rocket {
        Rocket { x: 0.0, y: 0.0, speed_x: 0.0, speed_y: 0.0, orientation, landed: false, engine_on: false }
    }

    #[test]
    fn rotates_clockwise() {
        let mut rocket = make_rocket(0.0);
        update_orientation(&mut rocket, 1.0, false, true);
        assert_eq!(rocket.orientation, 90.0);
    }

    #[test]
    fn rotates_counterclockwise() {
        let mut rocket = make_rocket(90.0);
        update_orientation(&mut rocket, 1.0, true, false);
        assert_eq!(rocket.orientation, 0.0);
    }

    #[test]
    fn wraps_past_360() {
        let mut rocket = make_rocket(350.0);
        update_orientation(&mut rocket, 1.0, false, true);
        assert!((rocket.orientation - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn wraps_below_zero() {
        let mut rocket = make_rocket(10.0);
        update_orientation(&mut rocket, 1.0, true, false);
        assert!((rocket.orientation - 280.0).abs() < f32::EPSILON);
    }

    #[test]
    fn no_change_when_no_keys() {
        let mut rocket = make_rocket(45.0);
        update_orientation(&mut rocket, 1.0, false, false);
        assert_eq!(rocket.orientation, 45.0);
    }

    #[test]
    fn both_keys_cancel_out() {
        let mut rocket = make_rocket(45.0);
        update_orientation(&mut rocket, 1.0, true, true);
        assert_eq!(rocket.orientation, 45.0);
    }

    #[test]
    fn full_rotation_in_four_seconds() {
        let mut rocket = make_rocket(0.0);
        update_orientation(&mut rocket, 4.0, false, true);
        assert!((rocket.orientation - 0.0).abs() < f32::EPSILON);
    }
}
