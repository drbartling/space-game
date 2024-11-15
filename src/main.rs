#![warn(
    clippy::cognitive_complexity,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::large_digit_groups
)]
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

mod bullet;
mod camera;
mod particle;
mod ship;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (camera::setup, ship::spawn))
        .add_systems(
            Update,
            (
                ship::update,
                ship::target,
                particle::update,
                bullet::update,
                camera::follow,
            ),
        );

    app.add_plugins(WorldInspectorPlugin::new());
    app.run();
}
