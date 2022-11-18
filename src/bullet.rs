use macroquad::prelude::*;

pub const SHOT_SPEED: f32 = 7.;
pub const SHOT_LIFE: f64 = 2.5;

#[derive(Default)]
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub shot_at: f64,
    pub collided: bool,
    pub color: Color,
}
