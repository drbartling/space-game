use std::f32::consts::TAU;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Particle {
    pub velocity: Vec2,
    pub wobble: f32,
    pub color: Color,
    pub life_time: Timer,
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.life_time.tick(time.delta());
        if particle.life_time.finished() {
            commands.entity(entity).despawn();
        }

        let dt = time.delta_seconds();
        transform.translation += particle.velocity.extend(0.0) * dt;

        let wobble =
            Vec2::from_angle(rand::random::<f32>() * TAU) * particle.wobble;
        particle.velocity += wobble;
        particle.wobble *= 0.05_f32.powf(dt);
    }
}
