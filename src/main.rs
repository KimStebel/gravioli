use macroquad::prelude::*;

const BALL_RADIUS: f32 = 80.0;

struct Ball {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

impl Ball {
    fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
        if self.y < screen_height() - BALL_RADIUS {
            self.speed_y += 0.2 * get_frame_time() * 60.0;
        } else {
            self.speed_y = 0.0;
            self.y = screen_height() - BALL_RADIUS;
        }

    }
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
    let mut ball = Ball {
        x: screen_width() / 2.0,
        y: screen_height() - BALL_RADIUS,
        speed_x: 0.0,
        speed_y: 0.0,
    };

    loop {
        if is_key_pressed(KeyCode::W) && ball.y >= screen_height() - BALL_RADIUS {
            ball.speed_y = -10.0;
        }

        ball.update();

        clear_background(BLACK);
        draw_circle(ball.x, ball.y, BALL_RADIUS, WHITE);
        next_frame().await;
    }
}
