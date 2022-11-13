#![warn(clippy::cognitive_complexity, clippy::unseparated_literal_suffix)]
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad_particles::{AtlasConfig, BlendMode, Emitter, EmitterConfig};
const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

#[derive(Default)]
struct Game {
    ship: Ship,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    game_over: bool,
}

impl Game {
    fn new(asteroid_count: usize) -> Self {
        let screen_center =
            Vec2::new(screen_width() / 2., screen_height() / 2.);
        let mut game = Self {
            ship: Ship {
                pos: screen_center,
                ..Default::default()
            },
            ..Default::default()
        };

        for _ in 0..asteroid_count {
            game.asteroids.push(Asteroid::rand())
        }
        game
    }
}

#[derive(Default)]
struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
    last_shot: f64,
}

#[derive(Default)]
struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

#[derive(Default)]
struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}
impl Asteroid {
    fn rand() -> Self {
        let screen_center =
            Vec2::new(screen_width() / 2., screen_height() / 2.);
        Asteroid {
            pos: screen_center
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                    .normalize()
                    * screen_width().min(screen_height())
                    / 2.,
            vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: rand::gen_range(4, 8),
            collided: false,
        }
    }
}

fn explosion() -> EmitterConfig {
    EmitterConfig {
        one_shot: true,
        emitting: false,
        lifetime: 0.3,
        lifetime_randomness: 0.7,
        explosiveness: 0.95,
        amount: 30,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        size: 30.0,
        gravity: vec2(0.0, -1000.0),
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height()
    }
    vr
}

#[macroquad::main("Asteroids")]
async fn main() {
    let texture = load_texture("assets/smoke_fire.png").await.unwrap();
    let mut game = Game::new(5);

    let mut one_shot_emitter = Emitter::new(EmitterConfig {
        texture: Some(texture),
        ..explosion()
    });

    loop {
        clear_background(BLACK);

        if game.game_over {
            let win = game.asteroids.is_empty();
            game = wait_for_new_game(win).await;
        }
        let frame_t = get_time();
        let rotation = game.ship.rot.to_radians();

        let mut acc = -game.ship.vel / 1000.; // Friction

        // Forward
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Up) {
            acc += Vec2::new(rotation.sin(), -rotation.cos()) / 5.;
        }
        // Back
        if is_key_down(KeyCode::E) || is_key_down(KeyCode::Down) {
            acc -= Vec2::new(rotation.sin(), -rotation.cos()) / 12.;
        }
        // Left
        if is_key_down(KeyCode::F) {
            acc -= Vec2::new(rotation.cos(), rotation.sin()) / 20.;
        }
        // Right
        if is_key_down(KeyCode::S) {
            acc += Vec2::new(rotation.cos(), rotation.sin()) / 20.;
        }

        // Shot
        if is_key_down(KeyCode::Space) && frame_t - game.ship.last_shot > 0.1 {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            game.bullets.push(Bullet {
                pos: game.ship.pos + rot_vec * SHIP_HEIGHT / 2.,
                vel: (rot_vec * 7.) + game.ship.vel,
                shot_at: frame_t,
                collided: false,
            });
            // Recoil
            acc -= Vec2::new(rotation.sin(), -rotation.cos()) / 10.;
            game.ship.last_shot = frame_t;
        }

        // Steer
        if is_key_down(KeyCode::Right) {
            game.ship.rot += 5.;
        } else if is_key_down(KeyCode::Left) {
            game.ship.rot -= 5.;
        }

        // Euler integration
        game.ship.vel += acc;
        game.ship.pos += game.ship.vel;
        game.ship.pos = wrap_around(&game.ship.pos);

        // Move each bullet
        for bullet in game.bullets.iter_mut() {
            bullet.pos += bullet.vel;
            bullet.pos = wrap_around(&bullet.pos);
        }

        // Move each asteroid
        for asteroid in game.asteroids.iter_mut() {
            asteroid.pos += asteroid.vel;
            asteroid.pos = wrap_around(&asteroid.pos);
            asteroid.rot += asteroid.rot_speed;
        }

        // Bullet lifetime
        game.bullets.retain(|bullet| bullet.shot_at + 2.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in game.asteroids.iter_mut() {
            // ship collision
            if (asteroid.pos - game.ship.pos).length()
                < asteroid.size + SHIP_HEIGHT / 3.
            {
                game.game_over = true;
                break;
            }

            // Asteroid/bullet collision
            for bullet in game.bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;
                    one_shot_emitter.draw(bullet.pos);
                    one_shot_emitter.config.emitting = true;

                    // Break the asteroid
                    if asteroid.sides > 3 {
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(bullet.vel.y, -bullet.vel.x)
                                .normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(-bullet.vel.y, bullet.vel.x)
                                .normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        })
                    }
                    break;
                }
            }
        }

        // Remove the collided objects
        game.bullets.retain(|bullet| {
            bullet.shot_at + 1.5 > frame_t && !bullet.collided
        });
        game.asteroids.retain(|asteroid| !asteroid.collided);
        game.asteroids.append(&mut new_asteroids);

        // You win?
        if game.asteroids.is_empty() {
            game.game_over = true;
        }

        if game.game_over {
            continue;
        }

        clear_background(BLACK);

        for bullet in game.bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., WHITE);
        }

        for asteroid in game.asteroids.iter() {
            draw_poly_lines(
                asteroid.pos.x,
                asteroid.pos.y,
                asteroid.sides,
                asteroid.size,
                asteroid.rot,
                2.,
                WHITE,
            )
        }

        let v1 = Vec2::new(
            game.ship.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
            game.ship.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            game.ship.pos.x
                - rotation.cos() * SHIP_BASE / 2.
                - rotation.sin() * SHIP_HEIGHT / 2.,
            game.ship.pos.y - rotation.sin() * SHIP_BASE / 2.
                + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            game.ship.pos.x + rotation.cos() * SHIP_BASE / 2.
                - rotation.sin() * SHIP_HEIGHT / 2.,
            game.ship.pos.y
                + rotation.sin() * SHIP_BASE / 2.
                + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., WHITE);
        root_ui().label(None, "hello megaui");
        if root_ui().button(None, "Push me") {
            println!("pushed");
        }
        next_frame().await
    }
}

async fn wait_for_new_game(win: bool) -> Game {
    loop {
        if win {
            show_win();
        } else {
            show_loss();
        }
        if is_key_down(KeyCode::Enter) {
            return Game::new(5);
        }
        next_frame().await;
        continue;
    }
}

fn show_win() {
    show_game_over("You Win!. Press [enter] to play again.");
}

fn show_loss() {
    show_game_over("Game Over. Press [enter] to play again.");
}

fn show_game_over(text: &str) {
    let font_size = 30.;
    let text_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height / 2.,
        font_size,
        LIGHTGRAY,
    );
}
