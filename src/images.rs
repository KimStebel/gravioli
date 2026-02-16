use macroquad::prelude::*;

pub struct Images {
    pub bg_texture: Texture2D,
    pub planet_texture: Texture2D,
}

impl Images {
    pub async fn load() -> Self {
        Self {
            bg_texture: load_texture("assets/backgrounds/space_backdrop.png").await.unwrap(),
            planet_texture: load_texture("assets/planets/planet05.png").await.unwrap(),
        }
    }
}
