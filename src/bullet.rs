use macroquad::prelude::*;

use crate::{game::GameObject, screen::wrap_around};

pub const SHOT_SPEED: f32 = 7.;
pub const SHOT_LIFE: f64 = 1.5;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub shot_at: f64,
    pub collided: bool,
    pub color: Color,
}

impl GameObject for Bullet {
    fn update(&mut self) -> Vec<crate::game::GameObjects> {
        self.pos += self.vel;
        self.pos = wrap_around(&self.pos);
        vec![]
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., self.color);
    }

    fn is_alive(&self) -> bool {
        !self.collided && get_time() - self.shot_at < SHOT_LIFE
    }
}
