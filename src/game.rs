use macroquad::prelude::*;

use crate::bullet::Bullet;
use crate::ship::Ship;
use crate::Asteroid;

#[derive(Default)]
pub struct Game {
    pub ship: Ship,
    pub enemy: Ship,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub game_over: bool,
}

impl Game {
    pub fn new(asteroid_count: usize) -> Self {
        let screen_center =
            Vec2::new(screen_width() / 2., screen_height() / 2.);
        Self {
            ship: Ship {
                pos: screen_center,
                health: 10,
                color: GREEN,
                ..Default::default()
            },
            enemy: Ship {
                pos: Vec2::new(
                    screen_width() * 3. / 4.,
                    screen_height() * 3. / 4.,
                ),
                health: 10,
                color: RED,
                ..Default::default()
            },
            asteroids: (0..asteroid_count).map(|_| Asteroid::rand()).collect(),
            ..Default::default()
        }
    }
}
