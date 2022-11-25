use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component, Default)]
pub struct Ship {
    /// Angular velocity in radians per second
    pub rotation: f32,
    /// Velocity vector in meters per second
    pub velocity: Vec2,
    /// Thrust in newtons
    pub thrust: f32,
    /// Mass in Kilograms
    pub mass: f32,
}

struct ShipControl {
    thrust_forward: bool,
    thrust_aft: bool,
    thrust_left: bool,
    thrust_right: bool,
    rotate_left: bool,
    rotate_right: bool,
    _fire: bool,
}

pub fn spawn(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(2.0),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn(GeometryBuilder::build_as(
            &shape,
            DrawMode::Stroke(StrokeMode {
                options: StrokeOptions::default()
                    .with_line_join(LineJoin::Round)
                    .with_end_cap(LineCap::Round)
                    .with_line_width(0.1),
                color: Color::RED,
            }),
            Transform::default().with_scale(Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.0,
            }),
        ))
        .insert(Ship {
            thrust: 60.0,
            mass: 100.0,
            ..Default::default()
        });
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Ship, &mut Transform)>,
) {
    let control = player_control(keys);
    let dt = time.delta_seconds();
    for (ship, transform) in query.iter_mut() {
        _update(ship, transform, &control, dt);
    }
}

fn _update(
    mut ship: Mut<Ship>,
    mut transform: Mut<Transform>,
    control: &ShipControl,
    dt: f32,
) {
    let dp = ship.velocity * dt;
    transform.translation += dp.extend(0.0);
    let dr = ship.rotation * dt;
    transform.rotate_z(dr);

    let mut thrust_vector = Vec2::ZERO;
    if control.thrust_forward {
        thrust_vector.y += 1.0;
    }
    if control.thrust_aft {
        thrust_vector.y -= 1.0;
    }
    if control.thrust_right {
        thrust_vector.x += 1.0;
    }
    if control.thrust_left {
        thrust_vector.x -= 1.0;
    }
    thrust_vector = (transform.rotation
        * (thrust_vector.extend(0.0) * ship.thrust))
        .truncate();
    let acc = thrust_vector / ship.mass;
    let dv = acc * dt;
    ship.velocity += dv;

    let mut rotation = 0.0;
    if control.rotate_left {
        rotation += 1.0;
    }
    if control.rotate_right {
        rotation -= 1.0;
    }
    rotation *= dt;
    ship.rotation += rotation;
}

fn player_control(keys: Res<Input<KeyCode>>) -> ShipControl {
    ShipControl {
        thrust_forward: keys.any_pressed([KeyCode::D, KeyCode::Up]),
        thrust_aft: keys.any_pressed([KeyCode::E, KeyCode::Down]),
        thrust_left: keys.any_pressed([KeyCode::F]),
        thrust_right: keys.any_pressed([KeyCode::S]),
        rotate_left: keys.any_pressed([KeyCode::Left]),
        rotate_right: keys.any_pressed([KeyCode::Right]),
        _fire: keys.any_pressed([KeyCode::Space]),
    }
}
