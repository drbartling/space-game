use macroquad::prelude::*;

use crate::{
    game::{GameObject, GameObjects},
    screen::wrap_around,
    ship,
};

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
    fn update(mut self, objs: &[GameObjects]) -> Vec<crate::game::GameObjects> {
        self.pos += self.vel;
        self.pos = wrap_around(&self.pos);
        self.collided = objs.iter().any(|o| match o {
            GameObjects::Ship(s) => {
                let dp = s.position - self.pos;
                let dist = dp.length().abs();
                dist < ship::SHIP_HEIGHT / 2.0
            }
            GameObjects::Particle(_) => false,
            GameObjects::Bullet(_) => false,
        });
        if self.is_alive() {
            vec![GameObjects::Bullet(self)]
        } else {
            vec![]
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., self.color);
    }

    fn is_alive(&self) -> bool {
        !self.collided && get_time() - self.shot_at < SHOT_LIFE
    }
}
