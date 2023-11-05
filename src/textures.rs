use macroquad::prelude::*;

pub struct Textures {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
}

impl Textures {
    pub async fn new(x:f32, y:f32, n: i32) -> Self {
        let path = format!("assets/textures/{}.png", n);
        Self {
            x,
            y,
            texture: load_texture(&path).await.unwrap(),
        }
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
