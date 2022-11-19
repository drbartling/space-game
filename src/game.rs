use macroquad::prelude::*;

use crate::{
    bullet, particle,
    ship::{self, player_control},
};

pub trait GameObject {
    fn update(&mut self) -> Vec<GameObjects>;
    fn draw(&self);
    fn is_alive(&self) -> bool;
}

#[derive(Default)]
pub struct Game {
    pub objects: Vec<GameObjects>,
}

impl Game {
    pub fn new() -> Self {
        let screen_center =
            Vec2::new(screen_width() / 2., screen_height() / 2.);

        let mut objects = vec![];
        let player_ship = ship::Ship {
            position: screen_center,
            health: 10,
            color: GREEN,
            controller: player_control,
            ..Default::default()
        };
        objects.push(GameObjects::Ship(player_ship));

        Self { objects }
    }
}

pub enum GameObjects {
    Ship(ship::Ship),
    Particle(particle::Particle),
    Bullet(bullet::Bullet),
}

impl GameObjects {
    pub fn update(&mut self) -> Vec<GameObjects> {
        match self {
            GameObjects::Ship(s) => s.update(),
            GameObjects::Particle(p) => p.update(),
            GameObjects::Bullet(b) => b.update(),
        }
    }
    pub fn draw(&self) {
        match self {
            GameObjects::Ship(s) => s.draw(),
            GameObjects::Particle(p) => p.draw(),
            GameObjects::Bullet(b) => b.draw(),
        }
    }
    pub fn is_alive(&self) -> bool {
        match self {
            GameObjects::Ship(s) => s.is_alive(),
            GameObjects::Particle(p) => p.is_alive(),
            GameObjects::Bullet(b) => b.is_alive(),
        }
    }
}
