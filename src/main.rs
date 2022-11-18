#![warn(
    clippy::cognitive_complexity,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::large_digit_groups
)]
mod bullet;
mod game;
mod screen;
mod ship;

use std::f32::consts::PI;

use game::Game;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use screen::wrap_around;
use ship::{ShipControl, SHIP_HEIGHT};

#[derive(Default)]
pub struct Asteroid {
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
        Asteroid {
            pos: screen::center()
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                    .normalize()
                    * screen_width().min(screen_height())
                    / 3.,
            vel: Vec2::from_angle(rand::gen_range(0.0, 2.0 * PI))
                * rand::gen_range(0.1, 1.0),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: rand::gen_range(6, 12),
            collided: false,
        }
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game = Game::new(0);

    loop {
        clear_background(BLACK);

        if game.game_over {
            let win = game.asteroids.is_empty();
            game = wait_for_new_game(win).await;
        }
        let frame_t = get_time();

        // let ship_control = ShipControl {
        //     thrust_forward: is_key_down(KeyCode::D) || is_key_down(KeyCode::Up),
        //     thrust_aft: is_key_down(KeyCode::E) || is_key_down(KeyCode::Down),
        //     thrust_left: is_key_down(KeyCode::S),
        //     thrust_right: is_key_down(KeyCode::F),
        //     rotate_left: is_key_down(KeyCode::Left),
        //     rotate_right: is_key_down(KeyCode::Right),
        //     fire: is_key_down(KeyCode::Space),
        // };

        let angle_to_enemy = game.ship.angle_to_deg(game.enemy.pos);
        let shot_angle = game
            .ship
            .shot_angle(game.enemy.pos, game.enemy.vel)
            .unwrap_or(angle_to_enemy);
        let speed = game
            .ship
            .vel
            .dot(Vec2::from_angle(game.ship.rot.to_radians()));
        let ship_control = if game.enemy.time_of_death.is_some() {
            ShipControl::default()
        } else {
            ShipControl {
                rotate_left: shot_angle > 1.,
                rotate_right: shot_angle < -1.,
                thrust_forward: angle_to_enemy.abs() < 30. && speed < 5.,
                thrust_aft: angle_to_enemy.abs() > 160. && speed > -5.,
                // thrust_left: -150. < angle_to_enemy && angle_to_enemy < 45.,
                // thrust_right: 45. < angle_to_enemy && angle_to_enemy < 150.,
                fire: shot_angle.abs() < 5.,
                ..Default::default()
            }
        };
        game.ship.update(ship_control, &mut game.bullets);

        let angle_to_ship = game.enemy.angle_to_deg(game.ship.pos);
        let enemy_speed = game
            .enemy
            .vel
            .dot(Vec2::from_angle(game.enemy.rot.to_radians()));
        let enemy_control = ShipControl {
            rotate_left: angle_to_ship > 0.,
            rotate_right: angle_to_ship < -0.,
            thrust_forward: angle_to_ship.abs() < 45. && enemy_speed < 4.,
            thrust_aft: angle_to_ship.abs() > 135. && enemy_speed > -4.,
            thrust_left: -120. < angle_to_ship && angle_to_ship < 60.,
            thrust_right: 60. < angle_to_ship && angle_to_ship < 120.,
            fire: angle_to_ship.abs() < 5.,
        };
        game.enemy.update(enemy_control, &mut game.bullets);

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
        game.bullets
            .retain(|bullet| bullet.shot_at + bullet::SHOT_LIFE > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in game.asteroids.iter_mut() {
            // ship collision
            if (asteroid.pos - game.ship.pos).length()
                < asteroid.size + SHIP_HEIGHT / 3.
            {
                game.ship.health -= 1;
                break_asteroid(asteroid, &mut new_asteroids, game.ship.vel);
                break;
            }
            if (asteroid.pos - game.enemy.pos).length()
                < asteroid.size + SHIP_HEIGHT / 3.
            {
                game.enemy.health -= 1;
                break_asteroid(asteroid, &mut new_asteroids, game.enemy.vel);
                break;
            }

            // Asteroid/bullet collision
            for bullet in game.bullets.iter_mut() {
                if (bullet.pos - game.ship.pos).length() < SHIP_HEIGHT / 3. {
                    game.ship.health -= 1;
                    bullet.collided = true;
                    break;
                }
                if (bullet.pos - game.enemy.pos).length() < SHIP_HEIGHT / 3. {
                    game.enemy.health -= 1;
                    bullet.collided = true;
                    break;
                }

                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    bullet.collided = true;
                    break_asteroid(asteroid, &mut new_asteroids, bullet.vel);
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
            game.asteroids.push(Asteroid::rand());
            // game.game_over = true;
        }

        if game.game_over {
            continue;
        }

        for bullet in game.bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., bullet.color);
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
        game.ship.draw();
        game.enemy.draw();
        let mut skin = root_ui().default_skin();
        let style = root_ui()
            .style_builder()
            .color(BLACK)
            .text_color(WHITE)
            .build();
        skin.label_style = style;
        root_ui().push_skin(&skin);

        root_ui().label(None, &format!("Health: {:.2}", game.ship.health));
        root_ui().label(None, &format!("Speed: {:.2}", speed));
        root_ui().label(None, &format!("Heat: {:.2}", game.ship.heat));
        if game.ship.overheated {
            root_ui().label(None, "Overheated!");
        }
        next_frame().await
    }
}

fn break_asteroid(
    asteroid: &mut Asteroid,
    new_asteroids: &mut Vec<Asteroid>,
    impact_vel: Vec2,
) {
    // Break the asteroid
    asteroid.collided = true;
    if asteroid.sides > 3 {
        new_asteroids.push(Asteroid {
            pos: asteroid.pos,
            vel: Vec2::new(impact_vel.y, -impact_vel.x).normalize()
                * rand::gen_range(1., 3.),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: asteroid.size * 0.8,
            sides: asteroid.sides - 1,
            collided: false,
        });
        new_asteroids.push(Asteroid {
            pos: asteroid.pos,
            vel: Vec2::new(-impact_vel.y, impact_vel.x).normalize()
                * rand::gen_range(1., 3.),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: asteroid.size * 0.8,
            sides: asteroid.sides - 1,
            collided: false,
        })
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
            return Game::new(1);
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
