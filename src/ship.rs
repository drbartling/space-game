use super::wrap_around;
use crate::bullet::{self, Bullet};
use macroquad::{prelude::*, rand::gen_range};

pub const SHIP_HEIGHT: f32 = 25.;
pub const SHIP_BASE: f32 = 22.;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ship {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: Vec2,
    pub last_shot: f64,
    pub color: Color,
    pub heat: f32,
    pub overheated: bool,
    pub health: i32,
    pub time_of_death: Option<f64>,
}

impl Ship {
    pub fn update(&mut self, control: ShipControl, bullets: &mut Vec<Bullet>) {
        let frame_t = get_time();
        let mut acc = -self.vel / 1000.; // Friction
        let heading_vector = self.heading_vector();

        if let Some(tod) = self.time_of_death {
            if 2. < frame_t - tod {
                *self = Ship {
                    color: self.color,
                    pos: Vec2::new(
                        gen_range(0., screen_width()),
                        gen_range(0., screen_height()),
                    ),
                    rot: -90.0,
                    health: 10,
                    ..Default::default()
                }
            }
            return;
        }
        if 0 > self.health {
            self.time_of_death = Some(frame_t);
            self.pos = Vec2::new(screen_width(), screen_height()) * 2.0;
            return;
        }

        if self.heat > 100. {
            self.overheated = true;
        }
        if self.heat < 20. {
            self.overheated = false;
        }

        if !self.overheated {
            if control.thrust_forward {
                acc += heading_vector / 5.;
                self.heat += 1.;
            }
            if control.thrust_aft {
                acc += -heading_vector / 12.;
                self.heat += 1.;
            }
            if control.thrust_left {
                acc += heading_vector.rotate(Vec2::Y) / 20.;
                self.heat += 1.;
            }
            if control.thrust_right {
                acc += -heading_vector.rotate(Vec2::Y) / 20.;
                self.heat += 1.;
            }
            if control.rotate_right {
                self.rot += 3.;
                self.heat += 1.;
            }
            if control.rotate_left {
                self.rot -= 3.;
                self.heat += 0.4;
            }

            // Shot
            if control.fire && frame_t - self.last_shot > 0.1 {
                self.heat += 1.;
                bullets.push(Bullet {
                    pos: self.pos + heading_vector * SHIP_HEIGHT / 2.,
                    vel: (heading_vector * bullet::SHOT_SPEED) + self.vel,
                    shot_at: frame_t,
                    collided: false,
                    color: self.color,
                });
                // Recoil
                acc -= heading_vector / 10.;
                self.last_shot = frame_t;
            }
        }

        self.heat *= 0.985;
        self.vel += acc;
        self.pos += self.vel;
        self.pos = wrap_around(&self.pos);
    }

    pub fn draw(&self) {
        if self.time_of_death.is_some() {
            return;
        }
        let rotation = self.rot.to_radians();
        let heading_vector = Vec2::from_angle(rotation);

        let v1 =
            Vec2::new(SHIP_HEIGHT / 2., 0.).rotate(heading_vector) + self.pos;
        let v2 = Vec2::new(-SHIP_HEIGHT / 2., SHIP_BASE / 2.)
            .rotate(heading_vector)
            + self.pos;
        let v3 = Vec2::new(-SHIP_HEIGHT / 2., -SHIP_BASE / 2.)
            .rotate(heading_vector)
            + self.pos;
        draw_triangle_lines(v1, v2, v3, 2., self.color);
    }

    pub fn heading_vector(&self) -> Vec2 {
        let rotation = self.rot.to_radians();
        Vec2::from_angle(rotation)
    }

    pub fn angle_to_deg(&self, pos: Vec2) -> f32 {
        let ship_angle = self.rot.to_radians();
        let heading_vector = Vec2::from_angle(ship_angle);
        let bearing_to_target = pos - self.pos;
        -heading_vector.angle_between(bearing_to_target).to_degrees()
    }

    pub fn shot_angle(
        &self,
        target_pos: Vec2,
        target_velocity: Vec2,
    ) -> Option<f32> {
        let rel_pos = target_pos - self.pos;
        let rel_vel = target_velocity - self.vel;
        let a =
            rel_vel.x.powi(2) + rel_vel.y.powi(2) - bullet::SHOT_SPEED.powi(2);
        let b = 2.0 * (rel_vel.x * rel_pos.x + rel_vel.y * rel_pos.y);
        let c = rel_pos.x.powi(2) + rel_pos.y.powi(2);
        let disc = b.powi(2) - 4.0 * a * c;
        if 0.0 > disc {
            return None;
        }

        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let t2 = (-b - disc.sqrt()) / (2.0 * a);
        let t = if t1 > 0.0 && t1 < t2 { t1 } else { t2 };
        if 0.0 > t || t as f64 > bullet::SHOT_LIFE * 60.0 * 2.0 {
            return None;
        }

        let predicted_pos = target_pos + rel_vel * t;
        draw_circle(predicted_pos.x, predicted_pos.y, 3.0, YELLOW);
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
        let result = ship.shot_angle(target_position, target_velocity);
        if let Some(expected_shot_angle) = expected {
            let result =
                result.expect(&format!("Was None, Should be {:?}", expected));
            assert_approx_eq!(f32, result, expected_shot_angle, epsilon = 0.01);
        } else {
            assert!(result.is_none());
        }
    }
}
