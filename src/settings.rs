pub const RES_WIDTH: i32 = 1600;
pub const RES_HEIGHT: i32 = 900;

//pub const HALF_WIDTH:f32 = (RES_WIDTH / 2) as f32;
pub const HALF_HEIGHT: f32 = (RES_HEIGHT / 2) as f32;

pub const PLAYER_START_POS_X: f32 = 1.5;
pub const PLAYER_START_POS_Y: f32 = 5.0;
pub const PLAYER_START_ANGLE: f32 = 5.0;
pub const PLAYER_ROTATION_SPEED: f32 = 4.0;
pub const PLAYER_SPEED: f32 = 3.0;

pub const FOV: f32 = 3.14159 / 3.0;
pub const HALF_FOV: f32 = FOV / 2.0;
pub const NUM_RAYS: i32 = RES_WIDTH / 2;
pub const DELTA_ANGLE: f32 = FOV / NUM_RAYS as f32;
pub const MAX_DEPTH: usize = 10;

//pub const SCREEN_DIST: f32 = HALF_WIDTH as f32 / HALF_FOV.tan();
pub const SCALE: i32 = RES_WIDTH / NUM_RAYS;

// pub const TEXTURE_SIZE:i32 = 256;
// pub const HALF_TEXTURE_SIZE: i32 = TEXTURE_SIZE / 2;
