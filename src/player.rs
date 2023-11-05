use macroquad::prelude::*;

use crate::{settings::{PLAYER_START_POS_X, PLAYER_START_POS_Y, PLAYER_START_ANGLE, PLAYER_SPEED, PLAYER_ROTATION_SPEED}, map::MAP};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    //pub m_x_pos: f32,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            x: PLAYER_START_POS_X,
            y: PLAYER_START_POS_Y,
            angle: PLAYER_START_ANGLE,
            //m_x_pos: 0.0,
        }
    }

    pub fn check_wall(&mut self, x: usize, y: usize) -> bool {
        if MAP[y][x] != 0 {
            true
        } else {
            false
        }
    }

    pub fn movement(&mut self) {
        let sin_a = self.angle.sin();
        let cos_a = self.angle.cos();
        let speed = PLAYER_SPEED * get_frame_time();
        let speed_sin = speed * sin_a;
        let speed_cos = speed * cos_a;
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if is_key_down(KeyCode::W) {
            dx += speed_cos;
            dy += speed_sin;
        }
        if is_key_down(KeyCode::S) {
            dx += -speed_cos;
            dy += -speed_sin;
        }
        if is_key_down(KeyCode::A) {
            dx += speed_sin;
            dy += -speed_cos;
        }
        if is_key_down(KeyCode::D) {
            dx += -speed_sin;
            dy += speed_cos;
        }

        if !self.check_wall((self.x + dx) as usize, (self.y + dy) as usize) {
            self.x += dx;
            self.y += dy;
        }

        if is_key_down(KeyCode::Left) {
            self.angle -= PLAYER_ROTATION_SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::Right) {
            self.angle += PLAYER_ROTATION_SPEED * get_frame_time();
        }
        self.angle %= 2.0 * 3.14159;
    }

    pub fn update(&mut self) {
        self.movement();
    }

    pub fn draw(&mut self) {
        //draw_line(self.x * 100.0, self.y * 100.0, self.x * 100.0 * RES_WIDTH as f32 * self.angle.cos(), self.y * 100.0 * RES_WIDTH as f32 * self.angle.sin(), 2.0, WHITE);
        //draw_circle(self.x * 100.0, self.y * 100.0, 15.0, GREEN);
    }
}