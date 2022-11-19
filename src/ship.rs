use crate::{
    bullet::{self, Bullet},
    game::{GameObject, GameObjects},
    particle::Particle,
};

use super::wrap_around;
use macroquad::prelude::*;

pub const SHIP_HEIGHT: f32 = 25.;
pub const SHIP_BASE: f32 = 22.;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ship {
    pub position: Vec2,
    pub rot: f32,
    pub vel: Vec2,
    pub last_shot: f64,
    pub color: Color,
    pub heat: f32,
    pub overheated: bool,
    pub health: i32,
    pub controller: fn() -> ShipControl,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rot: Default::default(),
            vel: Default::default(),
            last_shot: Default::default(),
            color: Default::default(),
            heat: Default::default(),
            overheated: Default::default(),
            health: Default::default(),
            controller: no_control,
        }
    }
}

fn no_control() -> ShipControl {
    ShipControl::default()
}

pub fn player_control() -> ShipControl {
    ShipControl {
        thrust_forward: is_key_down(KeyCode::D) || is_key_down(KeyCode::Up),
        thrust_aft: is_key_down(KeyCode::E) || is_key_down(KeyCode::Down),
        thrust_left: is_key_down(KeyCode::S),
        thrust_right: is_key_down(KeyCode::F),
        rotate_left: is_key_down(KeyCode::Left),
        rotate_right: is_key_down(KeyCode::Right),
        fire: is_key_down(KeyCode::Space),
    }
}

impl GameObject for Ship {
    fn update(mut self, _objs: &[GameObjects]) -> Vec<GameObjects> {
        self.position += self.vel;
        self.position = wrap_around(&self.position);

        let control = (self.controller)();
        let frame_t = get_time();
        let mut acc = -self.vel / 1000.; // Friction
        let heading_vector = self.heading_vector();
        let mut new_objects = vec![];

        if 0 > self.health {
            return vec![];
        }

        if self.heat > 100. {
            // self.overheated = true;
        }
        if self.heat < 20. {
            self.overheated = false;
        }

        let mut push_exhaust = |dv: Vec2, pos: Vec2| {
            let speed = dv.length();
            let p = Particle {
                position: self.position + pos,
                velocity: self.vel - dv * 10.0,
                color: ORANGE,
                spawn_time: get_time(),
                life_time: 2.0,
                wobble: speed,
            };
            new_objects.push(GameObjects::Particle(p));
        };

        if !self.overheated {
            if control.thrust_forward {
                let dv = heading_vector / 5.;
                acc += dv;
                self.heat += 1.;
                push_exhaust(dv, -heading_vector * SHIP_HEIGHT / 2.0);
            }
            if control.thrust_aft {
                let dv = -heading_vector / 12.;
                acc += dv;
                self.heat += 1.;
                push_exhaust(dv, heading_vector * SHIP_HEIGHT / 2.0);
            }
            if control.thrust_left {
                let dv = heading_vector.rotate(Vec2::Y) / 20.;
                acc += dv;
                self.heat += 1.;
                push_exhaust(dv, Vec2::ZERO);
            }
            if control.thrust_right {
                let dv = -heading_vector.rotate(Vec2::Y) / 20.;
                acc += dv;
                self.heat += 1.;
                push_exhaust(dv, Vec2::ZERO);
            }
            if control.rotate_right {
                self.rot += 3.;
                self.heat += 0.4;
            }
            if control.rotate_left {
                self.rot -= 3.;
                self.heat += 0.4;
            }
            if control.fire && frame_t - self.last_shot > 0.1 {
                self.heat += 1.;
                let bullet = Bullet {
                    pos: self.position + heading_vector * SHIP_HEIGHT / 2.,
                    vel: (heading_vector * bullet::SHOT_SPEED) + self.vel,
                    shot_at: frame_t,
                    collided: false,
                    color: self.color,
                };
                new_objects.push(GameObjects::Bullet(bullet));
                // Recoil
                acc -= heading_vector / 10.;
                self.last_shot = frame_t;
            }
        }

        self.heat *= 0.985;
        self.vel += acc;
        new_objects.push(GameObjects::Ship(self));
        new_objects
    }

    fn draw(&self) {
        let rotation = self.rot.to_radians();
        let heading_vector = Vec2::from_angle(rotation);

        let v1 = Vec2::new(SHIP_HEIGHT / 2., 0.).rotate(heading_vector)
            + self.position;
        let v2 = Vec2::new(-SHIP_HEIGHT / 2., SHIP_BASE / 2.)
            .rotate(heading_vector)
            + self.position;
        let v3 = Vec2::new(-SHIP_HEIGHT / 2., -SHIP_BASE / 2.)
            .rotate(heading_vector)
            + self.position;
        draw_triangle_lines(v1, v2, v3, 2., self.color);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl Ship {
    pub fn heading_vector(&self) -> Vec2 {
        let rotation = self.rot.to_radians();
        Vec2::from_angle(rotation)
    }

    pub fn angle_to_deg(&self, pos: Vec2) -> f32 {
        let ship_angle = self.rot.to_radians();
        let heading_vector = Vec2::from_angle(ship_angle);
        let bearing_to_target = pos - self.position;
        -heading_vector.angle_between(bearing_to_target).to_degrees()
    }

    pub fn shot_angle(
        &self,
        target_pos: Vec2,
        target_velocity: Vec2,
        projectile_speed: f32,
    ) -> Option<f32> {
        let rel_pos = target_pos - self.position;
        let rel_vel = target_velocity - self.vel;
        let a =
            rel_vel.x.powi(2) + rel_vel.y.powi(2) - projectile_speed.powi(2);
        let b = 2.0 * (rel_vel.x * rel_pos.x + rel_vel.y * rel_pos.y);
        let c = rel_pos.x.powi(2) + rel_pos.y.powi(2);
        let disc = b.powi(2) - 4.0 * a * c;
        if 0.0 > disc {
            return None;
        }

        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let t2 = (-b - disc.sqrt()) / (2.0 * a);
        let t = if t1 > 0.0 && t1 < t2 { t1 } else { t2 };
        if 0.0 > t || t as f64 > 5.0 * 60.0 {
            return None;
        }

        let predicted_pos = target_pos + rel_vel * t;
        if !cfg!(test) {
            draw_circle(predicted_pos.x, predicted_pos.y, 3.0, YELLOW);
        }
        Some(self.angle_to_deg(predicted_pos))
    }
}

#[derive(Default)]
pub struct ShipControl {
    pub thrust_forward: bool,
    pub thrust_aft: bool,
    pub thrust_left: bool,
    pub thrust_right: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub fire: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use rstest::rstest;

    #[rstest]
    #[case::zero_angles(Ship{rot:90., ..Default::default()}, Vec2::new(0.0, 10.0), 0.0)]
    #[case::right_side(Ship{rot:90., ..Default::default()}, Vec2::new(10.0, 0.0), 90.0)]
    #[case::left_side(Ship{rot:90., ..Default::default()}, Vec2::new(-10.0, 0.0), -90.0)]
    #[case::behind(Ship{rot:90., ..Default::default()}, Vec2::new(0.0, -10.0), -180.0)]
    #[case::behind_and_right(Ship{rot:90., ..Default::default()}, Vec2::new(10.0, -10.0), 135.0)]
    #[case::behind_and_left(Ship{rot:90., ..Default::default()}, Vec2::new(-10.0, -10.0), -135.0)]
    fn test_angle_to_deg(
        #[case] ship: Ship,
        #[case] pos: Vec2,
        #[case] expected_deg: f32,
    ) {
        let result = ship.angle_to_deg(pos);
        assert_approx_eq!(f32, result, expected_deg, epsilon = 0.01);
    }

    #[rstest]
    #[case::stationary_pointing_up_target_up(
        Ship{rot:90., ..Default::default()},
        Vec2::new(0.0, 10.0),
        Vec2::new(0.0, 0.0),
        Some(0.0),
    )]
    #[case::stationary_pointing_up_target_right(
        Ship{rot:90., ..Default::default()},
        Vec2::new(10.0, 0.0),
        Vec2::new(0.0, 0.0),
        Some(90.0),
    )]
    #[case::stationary_pointing_up_target_right_moving_target(
        Ship{rot:90., ..Default::default()},
        Vec2::new(10.0, 0.0),
        Vec2::new(0.0, 1.0),
        Some(81.79),
    )]
    #[case::stationary_pointing_up_target_right_fast_moving_target(
        Ship{rot:90., ..Default::default()},
        Vec2::new(10.0, 0.0),
        Vec2::new(0.0, 8.0),
        None,
    )]
    #[case::stationary_pointing_up_target_right(
        Ship{rot:90.,vel:Vec2::new(0.0, -1.0), ..Default::default()},
        Vec2::new(10.0, 0.0),
        Vec2::new(0.0, 0.0),
        Some(81.79),
    )]
    fn test_shot_angle(
        #[case] ship: Ship,
        #[case] target_position: Vec2,
        #[case] target_velocity: Vec2,
        #[case] expected: Option<f32>,
    ) {
        let result = ship.shot_angle(target_position, target_velocity, 7.);
        if let Some(expected_shot_angle) = expected {
            let result =
                result.expect(&format!("Was None, Should be {:?}", expected));
            assert_approx_eq!(f32, result, expected_shot_angle, epsilon = 0.01);
        } else {
            assert!(result.is_none());
        }
    }
}
