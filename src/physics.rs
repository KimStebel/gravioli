use macroquad::prelude::*;
use crate::state::{GameState, Planet, PlanetDef, Rocket, WinCondition};

pub enum PhysicsEvent {
    Collision,
    Win,
}

pub fn update(game: &mut GameState, dt: f32) -> Option<PhysicsEvent> {
    let current_planets = game.level.current_planets();
    for planet in &current_planets {
        apply_gravity(&mut game.level.rocket, planet, dt);
    }
    apply_engine(&mut game.level.rocket, dt);
    move_rocket(&mut game.level.rocket, dt);
    if current_planets.iter().any(|p| check_collision(&game.level.rocket, p)) {
        game.level.reset_rocket();
        return Some(PhysicsEvent::Collision);
    }
    if check_win(&game.level.rocket, &game.level.level.win_condition) {
        return Some(PhysicsEvent::Win);
    }
    None
}

fn check_win(rocket: &Rocket, condition: &WinCondition) -> bool {
    match condition {
        WinCondition::Circle { x, y, radius, max_speed } => {
            let dx = rocket.x - x;
            let dy = rocket.y - y;
            let in_circle = dx * dx + dy * dy < radius * radius;
            let speed = (rocket.speed_x * rocket.speed_x + rocket.speed_y * rocket.speed_y).sqrt();
            in_circle && speed < *max_speed && !rocket.engine_on
        }
        WinCondition::CircleAnySpeed { x, y, radius } => {
            let dx = rocket.x - x;
            let dy = rocket.y - y;
            dx * dx + dy * dy < radius * radius && !rocket.engine_on
        }
    }
}

fn apply_gravity(rocket: &mut Rocket, planet: &Planet, dt: f32) {
    let dx = planet.x - rocket.x;
    let dy = planet.y - rocket.y;
    let dist_sq = dx * dx + dy * dy;
    let dist = dist_sq.sqrt();
    let r = planet.radius / 30.0;
    let gravity = 4000000.0 * r * r * r;
    let accel = gravity / dist_sq;
    rocket.speed_x += (dx / dist) * accel * dt;
    rocket.speed_y += (dy / dist) * accel * dt;
}

fn apply_engine(rocket: &mut Rocket, dt: f32) {
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

pub fn project_path(rocket: &Rocket, planet_defs: &[PlanetDef], duration: f32, steps: usize, start_time: f64) -> Vec<(f32, f32)> {
    let dt = duration / steps as f32;
    let mut sim = rocket.clone();
    sim.engine_on = false;
    let mut path = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = start_time + (i as f64 + 1.0) * dt as f64;
        for def in planet_defs {
            let planet = def.planet_at(t);
            apply_gravity(&mut sim, &planet, dt);
        }
        move_rocket(&mut sim, dt);
        path.push((sim.x, sim.y));
    }
    path
}

pub fn check_collision(rocket: &Rocket, planet: &Planet) -> bool {
    let dx = rocket.x - planet.x;
    let dy = rocket.y - planet.y;
    dx * dx + dy * dy < planet.radius * planet.radius
}

#[cfg(test)]
mod tests {
    use crate::state::{Orbit, Rocket};
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
        apply_gravity(&mut rocket, &planet, 1.0);
        assert!(rocket.speed_x > 0.0);
        assert_eq!(rocket.speed_y, 0.0);
    }

    #[test]
    fn accelerates_toward_planet_above() {
        let mut rocket = make_rocket(0.0, 100.0, 0.0, 0.0);
        let planet = make_planet(0.0, 0.0);
        apply_gravity(&mut rocket, &planet, 1.0);
        assert_eq!(rocket.speed_x, 0.0);
        assert!(rocket.speed_y < 0.0);
    }

    #[test]
    fn stronger_gravity_when_closer() {
        let mut close = make_rocket(0.0, 0.0, 0.0, 0.0);
        let mut far = make_rocket(0.0, 0.0, 0.0, 0.0);
        let close_planet = make_planet(100.0, 0.0);
        let far_planet = make_planet(1000.0, 0.0);
        apply_gravity(&mut close, &close_planet, 1.0);
        apply_gravity(&mut far, &far_planet, 1.0);
        assert!(close.speed_x > far.speed_x);
    }

    #[test]
    fn zero_dt_no_acceleration() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        let planet = make_planet(100.0, 0.0);
        apply_gravity(&mut rocket, &planet, 0.0);
        assert_eq!(rocket.speed_x, 0.0);
        assert_eq!(rocket.speed_y, 0.0);
    }

    #[test]
    fn accelerates_diagonally() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        let planet = make_planet(100.0, 100.0);
        apply_gravity(&mut rocket, &planet, 1.0);
        assert!(rocket.speed_x > 0.0);
        assert!(rocket.speed_y > 0.0);
        assert!((rocket.speed_x - rocket.speed_y).abs() < f32::EPSILON);
    }

    #[test]
    fn larger_planet_stronger_gravity() {
        let mut small = make_rocket(0.0, 0.0, 0.0, 0.0);
        let mut large = make_rocket(0.0, 0.0, 0.0, 0.0);
        let small_planet = Planet { x: 100.0, y: 0.0, radius: 15.0 };
        let large_planet = Planet { x: 100.0, y: 0.0, radius: 30.0 };
        apply_gravity(&mut small, &small_planet, 1.0);
        apply_gravity(&mut large, &large_planet, 1.0);
        assert!(large.speed_x > small.speed_x);
    }

    #[test]
    fn gravity_proportional_to_radius_cubed() {
        let mut r30 = make_rocket(0.0, 0.0, 0.0, 0.0);
        let mut r60 = make_rocket(0.0, 0.0, 0.0, 0.0);
        let p30 = Planet { x: 100.0, y: 0.0, radius: 30.0 };
        let p60 = Planet { x: 100.0, y: 0.0, radius: 60.0 };
        apply_gravity(&mut r30, &p30, 1.0);
        apply_gravity(&mut r60, &p60, 1.0);
        // ratio should be (60/30)^3 = 8
        let ratio = r60.speed_x / r30.speed_x;
        assert!((ratio - 8.0).abs() < 0.001);
    }

    #[test]
    fn half_radius_one_eighth_gravity() {
        let mut full = make_rocket(0.0, 0.0, 0.0, 0.0);
        let mut half = make_rocket(0.0, 0.0, 0.0, 0.0);
        let full_planet = Planet { x: 100.0, y: 0.0, radius: 30.0 };
        let half_planet = Planet { x: 100.0, y: 0.0, radius: 15.0 };
        apply_gravity(&mut full, &full_planet, 1.0);
        apply_gravity(&mut half, &half_planet, 1.0);
        let ratio = half.speed_x / full.speed_x;
        assert!((ratio - 0.125).abs() < 0.001);
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
        apply_engine(&mut rocket, 1.0);
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
        apply_engine(&mut with_fuel, 1.0);
        apply_engine(&mut without_fuel, 1.0);
        // with fuel, thrust applies; without fuel, no thrust
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

    // --- engine_accel tests ---

    #[test]
    fn engine_accel_full_fuel() {
        let rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        // mass = 0.5 + 0.5 * (20/20) = 1.0, accel = 10/1 = 10
        assert!((engine_accel(&rocket) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn engine_accel_empty_fuel() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        rocket.fuel = 0.0;
        // mass = 0.5 + 0.5 * 0 = 0.5, accel = 10/0.5 = 20
        assert!((engine_accel(&rocket) - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn engine_accel_half_fuel() {
        let mut rocket = make_rocket(0.0, 0.0, 0.0, 0.0);
        rocket.fuel = 10.0;
        // mass = 0.5 + 0.5 * (10/20) = 0.75, accel = 10/0.75
        let expected = 10.0 / 0.75;
        assert!((engine_accel(&rocket) - expected).abs() < 0.001);
    }

    // --- check_win tests ---

    #[test]
    fn win_circle_inside_slow_engine_off() {
        let rocket = make_rocket(100.0, 100.0, 1.0, 0.0);
        let condition = WinCondition::Circle { x: 100.0, y: 100.0, radius: 50.0, max_speed: 2.0 };
        assert!(check_win(&rocket, &condition));
    }

    #[test]
    fn no_win_circle_too_fast() {
        let rocket = make_rocket(100.0, 100.0, 3.0, 0.0);
        let condition = WinCondition::Circle { x: 100.0, y: 100.0, radius: 50.0, max_speed: 2.0 };
        assert!(!check_win(&rocket, &condition));
    }

    #[test]
    fn no_win_circle_engine_on() {
        let mut rocket = make_rocket(100.0, 100.0, 1.0, 0.0);
        rocket.engine_on = true;
        let condition = WinCondition::Circle { x: 100.0, y: 100.0, radius: 50.0, max_speed: 2.0 };
        assert!(!check_win(&rocket, &condition));
    }

    #[test]
    fn no_win_circle_outside() {
        let rocket = make_rocket(200.0, 200.0, 0.0, 0.0);
        let condition = WinCondition::Circle { x: 100.0, y: 100.0, radius: 50.0, max_speed: 2.0 };
        assert!(!check_win(&rocket, &condition));
    }

    #[test]
    fn win_circle_any_speed_inside_engine_off() {
        let rocket = make_rocket(100.0, 100.0, 999.0, 999.0);
        let condition = WinCondition::CircleAnySpeed { x: 100.0, y: 100.0, radius: 50.0 };
        assert!(check_win(&rocket, &condition));
    }

    #[test]
    fn no_win_circle_any_speed_engine_on() {
        let mut rocket = make_rocket(100.0, 100.0, 0.0, 0.0);
        rocket.engine_on = true;
        let condition = WinCondition::CircleAnySpeed { x: 100.0, y: 100.0, radius: 50.0 };
        assert!(!check_win(&rocket, &condition));
    }

    #[test]
    fn no_win_circle_any_speed_outside() {
        let rocket = make_rocket(200.0, 200.0, 0.0, 0.0);
        let condition = WinCondition::CircleAnySpeed { x: 100.0, y: 100.0, radius: 50.0 };
        assert!(!check_win(&rocket, &condition));
    }

    // --- project_path tests ---

    fn make_static_planet_def(x: f32, y: f32, radius: f32) -> PlanetDef {
        PlanetDef { center_x: x, center_y: y, radius, orbit: None }
    }

    #[test]
    fn project_path_no_planets_straight_line() {
        let rocket = make_rocket(0.0, 0.0, 100.0, 0.0);
        let path = project_path(&rocket, &[], 1.0, 10, 0.0);
        assert_eq!(path.len(), 10);
        // should move right in a straight line
        for i in 1..path.len() {
            assert!(path[i].0 > path[i - 1].0);
            assert!((path[i].1).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn project_path_returns_correct_count() {
        let rocket = make_rocket(0.0, 0.0, 10.0, 0.0);
        let path = project_path(&rocket, &[], 2.0, 50, 0.0);
        assert_eq!(path.len(), 50);
    }

    #[test]
    fn project_path_static_planet_curves_trajectory() {
        let rocket = make_rocket(0.0, 0.0, 100.0, 0.0);
        let planets = vec![make_static_planet_def(0.0, 200.0, 10.0)];
        let path = project_path(&rocket, &planets, 2.0, 100, 0.0);
        // planet is below, so rocket should curve downward (positive y)
        let last = path.last().unwrap();
        assert!(last.1 > 0.0);
    }

    #[test]
    fn project_path_orbiting_planet_differs_from_static() {
        let rocket = make_rocket(0.0, 0.0, 100.0, 0.0);
        let static_planets = vec![make_static_planet_def(0.0, 200.0, 10.0)];
        let orbiting_planets = vec![PlanetDef {
            center_x: 0.0,
            center_y: 200.0,
            radius: 10.0,
            orbit: Some(Orbit { radius: 100.0, speed: 2.0, initial_angle: 0.0 }),
        }];
        let static_path = project_path(&rocket, &static_planets, 2.0, 100, 0.0);
        let orbiting_path = project_path(&rocket, &orbiting_planets, 2.0, 100, 0.0);
        // paths should diverge since the orbiting planet moves
        let last_static = static_path.last().unwrap();
        let last_orbit = orbiting_path.last().unwrap();
        let dx = last_static.0 - last_orbit.0;
        let dy = last_static.1 - last_orbit.1;
        assert!(dx * dx + dy * dy > 1.0);
    }

    #[test]
    fn project_path_disables_engine() {
        let mut rocket = make_rocket(0.0, 0.0, 100.0, 0.0);
        rocket.engine_on = true;
        let path = project_path(&rocket, &[], 1.0, 10, 0.0);
        // with engine disabled, should move in straight line (no thrust)
        for i in 1..path.len() {
            assert!((path[i].1).abs() < f32::EPSILON);
        }
    }
}
