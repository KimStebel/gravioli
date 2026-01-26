use macroquad::prelude::*;

const BALL_RADIUS: f32 = 100.0;

struct Ball {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

impl Ball {
    fn update(&mut self) {
        if self.y < screen_height() - BALL_RADIUS {
            self.speed_y += 0.1;
        } else {
            self.speed_y *= -0.8;
        }

        self.x += self.speed_x;
        self.y += self.speed_y;
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
        y: 600.0,
        speed_x: 0.0,
        speed_y: 0.0,
    };

    loop {
        ball.update();

        clear_background(BLACK);
        draw_circle(ball.x, ball.y, BALL_RADIUS, WHITE);
        next_frame().await;
    }
}
