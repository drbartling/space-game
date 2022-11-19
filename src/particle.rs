use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{game::GameObjects, screen::wrap_around};

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub wobble: f32,
    pub color: Color,
    pub spawn_time: f64,
    pub life_time: f64,
}

impl Particle {
    pub fn update(mut self, _objs: &[GameObjects]) -> Vec<GameObjects> {
        self.position += self.velocity;
        self.position = wrap_around(&self.position);
        let wobble =
            Vec2::from_angle(rand::gen_range(0.0, 2.0 * PI)) * self.wobble;
        self.velocity += wobble;
        self.wobble *= 0.9;
        self.color.a =
            1.0 - ((get_time() - self.spawn_time) / self.life_time) as f32;
        if self.is_alive() {
            vec![GameObjects::Particle(self)]
        } else {
            vec![]
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 1., self.color);
    }

    pub fn is_alive(&self) -> bool {
        get_time() - self.spawn_time < self.life_time
    }
}
