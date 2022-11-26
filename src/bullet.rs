use bevy::prelude::*;

use crate::ship::Ship;

pub const SHOT_SPEED: f32 = 30.;
pub const SHOT_LIFE: f32 = 1.5;

#[derive(Component, Default)]
pub struct Bullet {
    pub velocity: Vec2,
    pub life_time: Timer,
    pub color: Color,
    pub mass: f32,
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut ship_query: Query<(&mut Ship, &Transform), Without<Bullet>>,
    mut bullet_query: Query<
        (Entity, &mut Bullet, &mut Transform),
        Without<Ship>,
    >,
) {
    for (entity, mut bullet, mut bullet_transform) in bullet_query.iter_mut() {
        bullet.life_time.tick(time.delta());
        if bullet.life_time.finished() {
            commands.entity(entity).despawn();
        }

        let dt = time.delta_seconds();
        bullet_transform.translation += bullet.velocity.extend(0.0) * dt;
        for (mut ship, ship_transform) in ship_query.iter_mut() {
            let dp = (bullet_transform.translation
                - ship_transform.translation)
                .truncate();
            let distance = dp.length();
            if distance < ship.radius {
                commands.entity(entity).despawn();
                let dv = bullet.velocity - ship.velocity;
                let dot1 = -dp.dot(dv);
                let dot2 = dp.perp_dot(dv);
                let moment = dot1 * bullet.mass / ship.mass;
                ship.velocity += moment;
                let ang_moment = dot2 * bullet.mass / ship.mass;
                ship.rotation += ang_moment;
            }
        }
    }
}
