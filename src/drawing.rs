use macroquad::prelude::*;
use crate::state::{Planet, PlanetDef, Rocket, WinCondition};
use crate::images::Images;
use crate::physics;

pub fn draw(planet_defs: &[PlanetDef], rocket: &Rocket, win_condition: &WinCondition, images: &Images, elapsed: f64, show_hud: bool, show_path: bool) {
    let planets: Vec<_> = planet_defs.iter().map(|p| p.planet_at(elapsed)).collect();
    clear_background(BLACK);
    draw_texture_ex(&images.bg_texture, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(screen_width(), screen_height())),
        ..Default::default()
    });
    draw_win_condition(win_condition);
    for planet in &planets {
        draw_planet(planet, &images.planet_textures[planet.image]);
    }
    if show_path {
        draw_projected_path(rocket, planet_defs, elapsed);
    }
    draw_rocket(rocket);
    if show_hud {
        draw_hud(elapsed, rocket, &planets);
    }
    if elapsed < 5.0 {
        draw_help_text(&win_condition.description(), elapsed);
    }
}

fn draw_win_condition(condition: &WinCondition) {
    match condition {
        WinCondition::Circle { x, y, radius, .. }
        | WinCondition::CircleAnySpeed { x, y, radius } => {
            draw_circle_lines(*x, *y, *radius, 2.0, GREEN);
        }
    }
}

fn draw_planet(planet: &Planet, texture: &Texture2D) {
    let size = planet.radius * 2.0;
    draw_texture_ex(texture, planet.x - planet.radius, planet.y - planet.radius, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(size, size)),
        ..Default::default()
    });
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

    // Engine flames
    if rocket.engine_on {
        let flame_height = 15.0;
        let fl = rotate(-body_width / 3.0, 0.0);
        let fr = rotate(body_width / 3.0, 0.0);
        let ftip = rotate(0.0, flame_height);
        draw_triangle(fl.into(), fr.into(), ftip.into(), ORANGE);
    }
}

fn draw_projected_path(rocket: &Rocket, planet_defs: &[PlanetDef], elapsed: f64) {
    let path = physics::project_path(rocket, planet_defs, 5.0, 300, elapsed);
    for (x, y) in path.iter().step_by(15) {
        draw_circle(*x, *y, 1.5, WHITE);
    }
}

fn draw_hud(elapsed: f64, rocket: &Rocket, planets: &[Planet]) {
    let speed = (rocket.speed_x * rocket.speed_x + rocket.speed_y * rocket.speed_y).sqrt();
    let closest_dist = planets.iter().map(|p| {
        let dx = rocket.x - p.x;
        let dy = rocket.y - p.y;
        (dx * dx + dy * dy).sqrt() - p.radius
    }).reduce(f32::min);
    let x = screen_width() - 200.0;
    draw_text(&format!("FPS: {}", get_fps()), x, screen_height() - 120.0, 24.0, WHITE);
    if let Some(dist) = closest_dist {
        draw_text(&format!("Dist: {:.0} px", dist), x, screen_height() - 100.0, 24.0, WHITE);
    }
    draw_text(&format!("Time: {:.1}s", elapsed), x, screen_height() - 80.0, 24.0, WHITE);
    draw_text(&format!("Speed: {:.0} px/s", speed), x, screen_height() - 60.0, 24.0, WHITE);
    draw_text(&format!("Fuel: {:.1}s", rocket.fuel), x, screen_height() - 40.0, 24.0, WHITE);
    draw_text(&format!("Accel: {:.1} px/s²", physics::engine_accel(rocket)), x, screen_height() - 20.0, 24.0, WHITE);
}

pub fn draw_controls() {
    clear_background(BLACK);
    let title_size = 40.0;
    let font_size = 28.0;
    let line_height = 36.0;

    let title = "Controls";
    let dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text(title, (screen_width() - dims.width) / 2.0, 80.0, title_size, YELLOW);

    let controls = [
        ("A / D", "Rotate left / right"),
        ("Z", "Engine on"),
        ("X", "Engine off"),
        ("H", "Toggle HUD"),
        ("P", "Toggle trajectory path"),
        ("Escape", "Back to menu"),
    ];

    let start_y = 150.0;
    let key_x = screen_width() / 2.0 - 180.0;
    let desc_x = screen_width() / 2.0 - 20.0;

    for (i, (key, desc)) in controls.iter().enumerate() {
        let y = start_y + i as f32 * line_height;
        draw_text(key, key_x, y, font_size, WHITE);
        draw_text(desc, desc_x, y, font_size, GRAY);
    }

    let footer = "Press Escape to return";
    let dims = measure_text(footer, None, 24, 1.0);
    draw_text(footer, (screen_width() - dims.width) / 2.0, screen_height() - 40.0, 24.0, GRAY);
}

fn draw_help_text(text: &str, elapsed: f64) {
    let alpha = if elapsed > 4.0 { (5.0 - elapsed) as f32 } else { 1.0 };
    let color = Color::new(1.0, 1.0, 1.0, alpha);
    let font_size = 30.0;
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dimensions.width) / 2.0;
    let y = 50.0;
    draw_text(text, x, y, font_size, color);
}
