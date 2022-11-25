use bevy::{prelude::*, render::camera::ScalingMode};

pub fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(20.);
    commands.spawn(camera_bundle);
}
