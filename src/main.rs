use macroquad::prelude::*;
struct Planet {
    x: f32,
    y: f32,
    radius: f32
}
impl Planet {

}

struct Rocket {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
    orientation: f32, // degrees, 0/360 = up
    landed: bool
}

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
    let planet = Planet {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        radius: 30.0
    };

    let mut rocket = Rocket {
        x: 100.0,
        y: screen_height() - 100.0,
        speed_x: 200.0,
        speed_y: 0.0,
        orientation: 90.0,
        landed: false,
    };

    let start_time = get_time();
    let mut boosted = false;

    loop {
        let elapsed = get_time() - start_time;
        if !boosted && elapsed >= 5.0 {
            rocket.speed_x -= 120.0;
            rocket.speed_y += 60.0;
            boosted = true;
        }
        update_rocket_speed(&mut rocket, &planet);
        move_rocket(&mut rocket);
        clear_background(BLACK);
        draw_planet(&planet);
        draw_rocket(&rocket);
        draw_text(&format!("{:.1}s", elapsed), screen_width() - 80.0, screen_height() - 20.0, 24.0, WHITE);
        next_frame().await;
    }
}

fn update_rocket_speed(rocket: &mut Rocket, planet: &Planet) {
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

fn move_rocket(rocket: &mut Rocket) {
    let dt = get_frame_time();
    rocket.x += rocket.speed_x * dt;
    rocket.y += rocket.speed_y * dt;
}

fn draw_planet(planet: &Planet) {
    draw_circle(planet.x, planet.y, planet.radius, Color::new(0.76, 0.60, 0.42, 1.0));
}

fn draw_rocket(rocket: &Rocket) {
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
