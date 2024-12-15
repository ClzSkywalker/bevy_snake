use bevy::math::Vec2;
use rand::Rng;

pub static BLOCK_SIZE: Vec2 = Vec2::new(20., 20.);
pub static TIME_INTERVAL: f32 = 0.3;

pub static WINDOW_BORDER_START: Vec2 = Vec2::new(-5., -5.);
pub static WINDOW_BORDER_END: Vec2 = Vec2::new(5., 5.);

pub fn rand_pos(x_range: [i32; 2], y_range: [i32; 2], exclude: Vec<Vec2>) -> Vec2 {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(x_range[0] + 1..=x_range[1] - 1);
        let y = rng.gen_range(y_range[0] + 1..=y_range[1] - 1);
        let pos = Vec2::new(x as f32, y as f32);
        if !exclude.iter().any(|&item| item == pos) {
            return pos;
        }
    }
}
