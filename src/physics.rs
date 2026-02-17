use macroquad::prelude::*;
use crate::state::{GameState, Planet, Rocket};

pub fn update(game: &mut GameState, dt: f32) {
    for planet in &game.level.level.planets {
        update_rocket_speed(&mut game.level.rocket, planet, dt);
    }
    move_rocket(&mut game.level.rocket, dt);
    if game.level.level.planets.iter().any(|p| check_collision(&game.level.rocket, p)) {
        game.level.reset_rocket();
    }
}

pub fn update_rocket_speed(rocket: &mut Rocket, planet: &Planet, dt: f32) {
    let dx = planet.x - rocket.x;
    let dy = planet.y - rocket.y;
    let dist_sq = dx * dx + dy * dy;
    let dist = dist_sq.sqrt();
    let gravity = 4000000.0;
    let accel = gravity / dist_sq;
    rocket.speed_x += (dx / dist) * accel * dt;
    rocket.speed_y += (dy / dist) * accel * dt;

    if rocket.engine_on && rocket.fuel > 0.0 {
        apply_thrust(rocket, dt);
        rocket.fuel = (rocket.fuel - dt).max(0.0);
        if rocket.fuel == 0.0 {
            rocket.engine_on = false;
        }
    }
}

pub fn engine_accel(rocket: &Rocket) -> f32 {
    let force = 10.0;
    let mass = 0.5 + 0.5 * (rocket.fuel / 20.0);
    force / mass
}

pub fn apply_thrust(rocket: &mut Rocket, dt: f32) {
    let accel = engine_accel(rocket);
    let angle = rocket.orientation.to_radians();
    rocket.speed_x += angle.sin() * accel * dt;
    rocket.speed_y -= angle.cos() * accel * dt;
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
        Rocket { x, y, speed_x, speed_y, orientation: 0.0, landed: false, engine_on: false, fuel: 20.0 }
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

    #[test]
    fn thrust_acceleration_at_full_fuel() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        rocket.fuel = 20.0;
        apply_thrust(&mut rocket, 1.0);
        // mass = 0.5 + 0.5 * (20/20) = 1.0, accel = 10/1 = 10
        assert!((rocket.speed_y - -10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn thrust_acceleration_at_empty_fuel() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        rocket.fuel = 0.0;
        apply_thrust(&mut rocket, 1.0);
        // mass = 0.5 + 0.5 * (0/20) = 0.5, accel = 10/0.5 = 20
        assert!((rocket.speed_y - -20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn thrust_acceleration_at_half_fuel() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        rocket.fuel = 10.0;
        apply_thrust(&mut rocket, 1.0);
        // mass = 0.5 + 0.5 * (10/20) = 0.75, accel = 10/0.75 = 13.333...
        let expected = 10.0 / 0.75;
        assert!((rocket.speed_y - -expected).abs() < 0.001);
    }

    #[test]
    fn thrust_stronger_with_less_fuel() {
        let mut full = make_rocket(0.0, 0.0, 0.0, 0.0);
        full.fuel = 20.0;
        let mut low = make_rocket(0.0, 0.0, 0.0, 0.0);
        low.fuel = 5.0;
        apply_thrust(&mut full, 1.0);
        apply_thrust(&mut low, 1.0);
        assert!(low.speed_y.abs() > full.speed_y.abs());
    }

    #[test]
    fn engine_off_when_fuel_runs_out() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        rocket.engine_on = true;
        rocket.fuel = 0.5;
        let planet = make_planet(0.0, 10000.0);
        update_rocket_speed(&mut rocket, &planet, 1.0);
        assert_eq!(rocket.fuel, 0.0);
        assert!(!rocket.engine_on);
    }

    #[test]
    fn no_thrust_when_no_fuel() {
        let mut with_fuel = make_rocket(0.0, 0.0, 0.0, 0.0);
        with_fuel.engine_on = true;
        with_fuel.fuel = 20.0;
        let mut without_fuel = make_rocket(0.0, 0.0, 0.0, 0.0);
        without_fuel.engine_on = true;
        without_fuel.fuel = 0.0;
        let planet = make_planet(0.0, 10000.0);
        update_rocket_speed(&mut with_fuel, &planet, 1.0);
        update_rocket_speed(&mut without_fuel, &planet, 1.0);
        // with fuel, thrust opposes gravity so less positive speed_y
        assert!(with_fuel.speed_y < without_fuel.speed_y);
    }

    #[test]
    fn collision_inside_planet() {
        let rocket = make_rocket(100.0, 100.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        assert!(check_collision(&rocket, &planet));
    }

    #[test]
    fn collision_near_center() {
        let rocket = make_rocket(105.0, 100.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        assert!(check_collision(&rocket, &planet));
    }

    #[test]
    fn no_collision_outside_planet() {
        let rocket = make_rocket(200.0, 200.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        assert!(!check_collision(&rocket, &planet));
    }

    #[test]
    fn no_collision_on_boundary() {
        let rocket = make_rocket(130.0, 100.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        assert!(!check_collision(&rocket, &planet));
    }

    #[test]
    fn collision_just_inside() {
        let rocket = make_rocket(129.0, 100.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        assert!(check_collision(&rocket, &planet));
    }
}
