use bevy::prelude::*;

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
    mut query: Query<(Entity, &mut Bullet, &mut Transform)>,
) {
    for (entity, mut bullet, mut transform) in query.iter_mut() {
        bullet.life_time.tick(time.delta());
        if bullet.life_time.finished() {
            commands.entity(entity).despawn();
        }

        let dt = time.delta_seconds();
        transform.translation += bullet.velocity.extend(0.0) * dt;
    }
}
