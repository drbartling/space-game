use macroquad::prelude::*;

use crate::{
    bullet, particle,
    screen::center,
    ship::{self, player_control},
};

pub trait GameObject {
    fn update(self, objs: &[GameObjects]) -> Vec<GameObjects>;
    fn draw(&self);
    fn is_alive(&self) -> bool;
}

#[derive(Default)]
pub struct Game {
    pub objects: Vec<GameObjects>,
}

impl Game {
    pub fn new() -> Self {
        let mut objects = vec![];

        let player_ship = ship::Ship {
            position: center(),
            health: 10,
            color: GREEN,
            controller: player_control,
            ..Default::default()
        };
        objects.push(GameObjects::Ship(player_ship));

        let enemy_ship = ship::Ship {
            position: center() * 1.5,
            health: 10,
            color: RED,
            ..Default::default()
        };
        objects.push(GameObjects::Ship(enemy_ship));

        Self { objects }
    }
}

pub enum GameObjects {
    Ship(ship::Ship),
    Particle(particle::Particle),
    Bullet(bullet::Bullet),
}

impl GameObjects {
    pub fn update(self, objs: &[GameObjects]) -> Vec<GameObjects> {
        match self {
            GameObjects::Ship(s) => s.update(objs),
            GameObjects::Particle(p) => p.update(objs),
            GameObjects::Bullet(b) => b.update(objs),
        }
    }
    pub fn draw(&self) {
        match self {
            GameObjects::Ship(s) => s.draw(),
            GameObjects::Particle(p) => p.draw(),
            GameObjects::Bullet(b) => b.draw(),
        }
    }
}
