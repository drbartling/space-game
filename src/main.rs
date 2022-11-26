#![warn(
    clippy::cognitive_complexity,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::large_digit_groups
)]
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

mod camera;
mod ship;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera::setup)
        .add_startup_system(ship::spawn)
        .add_system(ship::update)
        .add_system(camera::follow)
        .run();
}
