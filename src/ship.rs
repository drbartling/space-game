use std::f32::consts::TAU;

use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    bullet::{Bullet, SHOT_LIFE, SHOT_SPEED},
    particle::Particle,
};

#[derive(Component)]
pub struct Ship {
    pub id: usize,
    /// Angular velocity in radians per second
    pub rotation: f32,
    /// Velocity vector in meters per second
    pub velocity: Vec2,
    /// Thrust in newtons
    pub thrust: f32,
    /// Mass in Kilograms
    pub mass: f32,
    pub cannon_timer: Timer,
    pub color: Color,
    pub controller: fn(&Res<ButtonInput<KeyCode>>) -> ShipControl,
    pub radius: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            id: Default::default(),
            rotation: Default::default(),
            velocity: Default::default(),
            thrust: Default::default(),
            mass: Default::default(),
            cannon_timer: Default::default(),
            color: Default::default(),
            controller: default_controller,
            radius: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct ShipControl {
    thrust_forward: bool,
    thrust_aft: bool,
    thrust_left: bool,
    thrust_right: bool,
    rotate_left: bool,
    rotate_right: bool,
    fire: bool,
}

#[derive(Resource)]
pub struct PlayerShip(pub usize);

pub fn spawn(mut commands: Commands) {
    let radius = 1.0;
    let ship_id = rand::random();
    let ship_color = css::GREEN.into();

    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(2.0),
        ..shapes::RegularPolygon::default()
    };

    // Player ship
    commands
        .spawn(GeometryBuilder::build_as(&shape))
        .insert(Ship {
            id: ship_id,
            thrust: 1000.0,
            mass: 100.0,
            cannon_timer: Timer::from_seconds(0.1, TimerMode::Once),
            color: ship_color,
            controller: player_control,
            radius,
            ..Default::default()
        });
    commands.insert_resource(PlayerShip(ship_id));

    // Enemy ship
    let ship_id = rand::random();
    let ship_color = css::RED.into();
    commands
        .spawn(GeometryBuilder::build_as(&shape))
        .insert(Ship {
            id: ship_id,
            thrust: 1000.0,
            mass: 100.0,
            cannon_timer: Timer::from_seconds(0.1, TimerMode::Once),
            color: ship_color,
            controller: hillarius,
            radius,
            ..Default::default()
        });
}

pub fn update(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Ship, &mut Transform)>,
) {
    for (mut ship, mut transform) in query.iter_mut() {
        let control = (ship.controller)(&keys);
        let dt = time.delta_seconds();
        let dp = ship.velocity * dt;
        transform.translation += dp.extend(0.0);
        let dr = ship.rotation * dt;
        transform.rotate_z(dr);

        ship.cannon_timer.tick(time.delta());
        if ship.cannon_timer.finished() && control.fire {
            ship.cannon_timer.reset();

            let rel_velocity = (transform.rotation
                * (Vec2::Y * SHOT_SPEED).extend(0.0))
            .truncate();
            let mass = 1.0;
            let recoil = -rel_velocity * mass / ship.mass;
            ship.velocity += recoil;
            let velocity = rel_velocity + ship.velocity;

            let b = Bullet {
                velocity,
                life_time: Timer::from_seconds(SHOT_LIFE, TimerMode::Once),
                color: ship.color,
                mass,
            };

            let pos = (transform.rotation * Vec2::new(0.0, 2.0).extend(0.0))
                .truncate();

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: ship.color,
                        custom_size: Some(Vec2::new(0.1, 0.1)),
                        ..default()
                    },
                    transform: Transform {
                        translation: transform.translation + pos.extend(0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(b);
        }

        let mut push_exhaust = |dv: Vec2, pos: Vec2| {
            let dv = (transform.rotation * dv.extend(0.0)).truncate();
            let pos = (transform.rotation * pos.extend(0.0)).truncate();
            let speed = dv.length();
            let p = Particle {
                velocity: ship.velocity - dv * 10.0,
                color: css::ORANGE.into(),
                life_time: Timer::from_seconds(0.5, TimerMode::Once),
                wobble: speed,
            };
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: css::ORANGE.into(),
                        custom_size: Some(Vec2::new(0.1, 0.1)),
                        ..default()
                    },
                    transform: Transform {
                        translation: transform.translation + pos.extend(0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(p);
        };

        let mut thrust_vector = Vec2::ZERO;
        if control.thrust_forward {
            let dv = Vec2::new(0.0, 1.0);
            let pos = Vec2::new(0.0, -1.0);
            thrust_vector += dv;
            push_exhaust(dv, pos);
        }
        if control.thrust_aft {
            let dv = Vec2::new(0.0, -0.5);
            let pos = Vec2::new(0.0, 2.0);
            thrust_vector += dv;
            push_exhaust(dv, pos);
        }
        if control.thrust_right {
            let dv = Vec2::new(0.3, 0.0);
            let pos = Vec2::new(-0.6, 0.0);
            thrust_vector += dv;
            push_exhaust(dv, pos);
        }
        if control.thrust_left {
            let dv = Vec2::new(-0.3, 0.0);
            let pos = Vec2::new(0.6, 0.0);
            thrust_vector += dv;
            push_exhaust(dv, pos);
        }
        thrust_vector = (transform.rotation
            * (thrust_vector.extend(0.0) * ship.thrust))
            .truncate();
        let acc = thrust_vector / ship.mass;
        let dv = acc * dt;
        ship.velocity += dv;

        let mut rotation = 0.0;
        if control.rotate_left {
            rotation += TAU;
        }
        if control.rotate_right {
            rotation -= TAU;
        }
        rotation *= dt;
        ship.rotation += rotation;

        ship.rotation *= 0.1_f32.powf(dt);
        ship.velocity *= 0.95_f32.powf(dt);
    }
}

fn player_control(keys: &Res<ButtonInput<KeyCode>>) -> ShipControl {
    ShipControl {
        thrust_forward: keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowUp]),
        thrust_aft: keys.any_pressed([KeyCode::KeyE, KeyCode::ArrowDown]),
        thrust_left: keys.any_pressed([KeyCode::KeyF]),
        thrust_right: keys.any_pressed([KeyCode::KeyS]),
        rotate_left: keys.any_pressed([KeyCode::ArrowLeft]),
        rotate_right: keys.any_pressed([KeyCode::ArrowRight]),
        fire: keys.any_pressed([KeyCode::Space]),
    }
}

fn hillarius(_keys: &Res<ButtonInput<KeyCode>>) -> ShipControl {
    ShipControl {
        thrust_forward: true,
        fire: true,
        rotate_right: true,
        thrust_aft: true,
        thrust_left: true,
        thrust_right: true,
        rotate_left: false,
    }
}

fn default_controller(_keys: &Res<ButtonInput<KeyCode>>) -> ShipControl {
    ShipControl::default()
}

pub fn target(
    player_id: Option<Res<PlayerShip>>,
    query: Query<(&Ship, &Transform)>,
) {
    let id = if let Some(id) = player_id {
        id.0
    } else {
        return;
    };
    let mut player_transform = None;
    let mut other_transform = None;
    for (ship, transform) in query.iter() {
        if ship.id == id {
            player_transform = Some(transform);
        } else {
            other_transform = Some(transform);
        }
    }
    if let Some(player_transform) = player_transform {
        if let Some(other_transform) = other_transform {
            let dp = (other_transform.translation
                - player_transform.translation)
                .truncate();
            let mut dp =
                (player_transform.rotation * dp.extend(0.0)).truncate();
            dp.y *= -1.0;
            // let dp = dp.normalize();
            dbg!(dp);
        }
    }
}
