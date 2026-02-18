use macroquad::prelude::*;

pub struct Images {
    pub bg_texture: Texture2D,
    pub planet_textures: Vec<Texture2D>,
}

impl Images {
    pub async fn load() -> Self {
        let mut planet_textures = Vec::new();
        for i in 0..10 {
            let path = format!("assets/planets/planet{:02}.png", i);
            planet_textures.push(load_texture(&path).await.unwrap());
        }
        Self {
            bg_texture: load_texture("assets/backgrounds/spr_stars01.png").await.unwrap(),
            planet_textures,
        }
    }
}
