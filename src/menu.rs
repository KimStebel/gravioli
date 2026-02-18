use macroquad::prelude::*;
use crate::state::Level;

pub enum MenuChoice {
    Play(usize),
    Exit,
}

#[derive(Clone)]
pub struct Menu {
    selected: usize,
    options: Vec<&'static str>,
}

impl Menu {
    pub fn new(levels: &Vec<Level>) -> Self {
        let level_names: Vec<&str> = levels.iter().map(|l| l.name).collect();

        let mut options = level_names;
        options.push("Exit");
        Self { selected: 0, options }
    }

    pub fn update(&mut self) -> Option<MenuChoice> {
        let choice = self.handle_input();
        if choice.is_none() {
            self.draw();
        }
        choice
    }

    fn handle_input(&mut self) -> Option<MenuChoice> {
        if is_key_pressed(KeyCode::Up) && self.selected > 0 {
            self.selected -= 1;
        }
        if is_key_pressed(KeyCode::Down) && self.selected < self.options.len() - 1 {
            self.selected += 1;
        }
        if is_key_pressed(KeyCode::Enter) {
            return match self.selected {
                i if i == self.options.len() - 1 => Some(MenuChoice::Exit),
                i => Some(MenuChoice::Play(i)),
            };
        }
        None
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        let font_size = 32.0;
        let line_height = 40.0;
        let start_y = screen_height() / 2.0 - (self.options.len() as f32 * line_height) / 2.0;

        for (i, option) in self.options.iter().enumerate() {
            let color = if i == self.selected { YELLOW } else { WHITE };
            let text = if i == self.selected {
                format!("> {}", option)
            } else {
                format!("  {}", option)
            };
            let dims = measure_text(&text, None, font_size as u16, 1.0);
            let x = (screen_width() - dims.width) / 2.0;
            draw_text(&text, x, start_y + i as f32 * line_height, font_size, color);
        }
    }
}
