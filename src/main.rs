#![warn(
    clippy::cognitive_complexity,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::large_digit_groups
)]
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod bullet;
mod camera;
mod particle;
mod ship;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(camera::setup)
        .add_startup_system(ship::spawn)
        .add_system(ship::update)
        .add_system(ship::target)
        .add_system(particle::update)
        .add_system(bullet::update)
        .add_system(camera::follow);

    app.add_plugin(WorldInspectorPlugin::new());
    app.run()
}
