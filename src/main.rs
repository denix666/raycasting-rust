use macroquad::prelude::*;

mod settings;
use settings::*;

mod player;
use player::Player;

// mod textures;
// use textures::Textures;

mod map;
use map::{Map, MAP};

fn window_conf() -> Conf {
    let mut title = String::from("Shooter v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: RES_WIDTH,
        window_height: RES_HEIGHT,
        ..Default::default()
    }
}

pub fn check_wall(x: usize, y: usize) -> bool {
    if x > 21 {
        return false
    }
    if y > 8 {
        return false
    }

    if MAP[y][x] != 0 {
        true
    } else {
        false
    }
}

pub fn texture_type(x: usize, y: usize) -> i32 {
    MAP[y][x]
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut map = Map::new().await;
    let mut player = Player::new().await;
    //let mut textures: Vec<Textures> = Vec::new();

    loop {
        clear_background(BLACK);

        map.draw();
        player.update();
        player.draw();

        //textures.clear();

        let ox = player.x;
        let oy = player.y;
        let x_map = ox as i32;
        let y_map = oy as i32;
        let mut ray_angle = player.angle - HALF_FOV + 0.0001;

        // let mut texture_vert: i32 = 0;
        // let mut texture_hor: i32 = 0;
        // let mut texture: i32 = 0;
        let mut offset: f32 = 0.0;

        for ray in 0..NUM_RAYS {
            let sin_a = ray_angle.sin();
            let cos_a = ray_angle.cos();

            let mut x_vert: f32;
            let mut y_vert: f32;
            let mut y_hor: f32;
            let mut x_hor: f32;
            let mut dy: f32;
            let mut dx: f32;
            let mut depth_vert: f32;
            let mut delta_depth: f32;
            let mut depth_hor: f32;
            let mut depth: f32;

            // horizontals
            if sin_a > 0.0 {
                y_hor = y_map as f32 + 1.0;
                dy = 1.0;
            } else {
                y_hor = y_map as f32 - 0.000001;
                dy = -1.0;
            }

            depth_hor = (y_hor - oy) / sin_a;
            x_hor = ox + depth_hor * cos_a;

            delta_depth = dy / sin_a;
            dx = delta_depth * cos_a;

            for _ in 0..MAX_DEPTH {
                if check_wall(x_hor as usize, y_hor as usize) {
                    //texture_hor = texture_type(x_hor as usize, y_hor as usize);
                    break
                }
                x_hor += dx;
                y_hor += dy;
                depth_hor += delta_depth;
            }

            // verticals
            if cos_a > 0.0 {
                x_vert = x_map as f32 + 1.0;
                dx = 1.0;
            } else {
                x_vert = x_map as f32 - 0.000001;
                dx = -1.0;
            }

            depth_vert = (x_vert - ox) / cos_a;
            y_vert = oy + depth_vert * sin_a;

            delta_depth = dx / cos_a;
            dy = delta_depth * sin_a;

            for _ in 0..MAX_DEPTH {
                if check_wall(x_vert as usize, y_vert as usize) {
                    //texture_vert = texture_type(x_vert as usize, y_vert as usize);
                    break
                }
                x_vert += dx;
                y_vert += dy;
                depth_vert += delta_depth;
            }
            
            if depth_vert < depth_hor {
                depth = depth_vert;
                //texture = texture_vert;
                y_vert %= 1.0;
                if cos_a > 0.0 {
                    offset = y_vert;
                } else {
                    offset = 1.0 - y_vert;
                }
            } else {
                depth = depth_hor;
                //texture = texture_hor;
                x_hor %= 1.0;
                if sin_a > 0.0 {
                    offset = 1.0 - x_hor;
                } else {
                    offset = x_hor;
                }
            }

            // remove fishbowl effect
            depth *= (player.angle - ray_angle).cos();

            // projection
            let proj_height = 1385.640646055102 / (depth + 0.0001);

            // DRAW 2D
            //draw_line(100.0 * ox, 100.0 * oy, 100.0 * ox + 100.0 * depth * cos_a, 100.0 * oy + 100.0 * depth * sin_a, 2.0, GRAY);

            // DRAW 3D projection
            let color = Color::new(depth / 14.5, depth / 14.5, depth / 14.5, 1.00);
            draw_rectangle((ray * SCALE) as f32, HALF_HEIGHT - proj_height / 2.0, SCALE as f32, proj_height, color);

            // textures.push(
            //     Textures::new(depth, proj_height, texture, offset).await,
            // );

            ray_angle += DELTA_ANGLE;
        }

        // for i in &mut textures {
        //     i.draw();
        // }

        next_frame().await
    }
}