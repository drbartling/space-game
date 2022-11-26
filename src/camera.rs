use bevy::{prelude::*, render::camera::ScalingMode};

use crate::ship::{PlayerShip, Ship};

pub fn setup(mut commands: Commands) {
    commands.spawn(new());
}

pub fn new() -> Camera2dBundle {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(20.);
    camera_bundle.transform.translation.x = 15.0;
    camera_bundle
}

pub fn follow(
    player_id: Option<Res<PlayerShip>>,
    ship_query: Query<(&Ship, &Transform)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Ship>)>,
) {
    let id = if let Some(id) = player_id {
        id.0
    } else {
        return;
    };

    for (ship, ship_transform) in ship_query.iter() {
        if ship.id != id {
            continue;
        }

        let pos = ship_transform.translation;

        for mut transform in camera_query.iter_mut() {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
            transform.rotation = ship_transform.rotation;
        }
    }
}
