#![warn(
    clippy::cognitive_complexity,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::large_digit_groups
)]
use macroquad::prelude::*;
use macroquad::ui::root_ui;
const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

#[derive(Default)]
struct Game {
    ship: Ship,
    enemy: Ship,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    game_over: bool,
}

impl Game {
    fn new(asteroid_count: usize) -> Self {
        let screen_center =
            Vec2::new(screen_width() / 2., screen_height() / 2.);
        Self {
            ship: Ship {
                pos: screen_center,
                color: WHITE,
                ..Default::default()
            },
            enemy: Ship {
                pos: Vec2::new(
                    screen_width() * 3. / 4.,
                    screen_height() * 3. / 4.,
                ),
                color: RED,
                ..Default::default()
            },
            asteroids: (0..asteroid_count).map(|_| Asteroid::rand()).collect(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
    last_shot: f64,
    color: Color,
}

impl Ship {
    fn update(&mut self, control: ShipControl, bullets: &mut Vec<Bullet>) {
        let frame_t = get_time();
        let mut acc = self.vel / 1000.; // Friction
        let rotation = self.rot.to_radians();

        if control.thrust_forward {
            acc += Vec2::new(rotation.sin(), -rotation.cos()) / 5.;
        }
        if control.thrust_aft {
            acc -= Vec2::new(rotation.sin(), -rotation.cos()) / 12.;
        }
        if control.thrust_left {
            acc -= Vec2::new(rotation.cos(), rotation.sin()) / 20.;
        }
        if control.thrust_right {
            acc += Vec2::new(rotation.cos(), rotation.sin()) / 20.;
        }
        if control.rotate_right {
            self.rot += 5.;
        }
        if control.rotate_left {
            self.rot -= 5.;
        }

        // Shot
        if control.fire && frame_t - self.last_shot > 0.1 {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            bullets.push(Bullet {
                pos: self.pos + rot_vec * SHIP_HEIGHT / 2.,
                vel: (rot_vec * 7.) + self.vel,
                shot_at: frame_t,
                collided: false,
            });
            // Recoil
            acc -= Vec2::new(rotation.sin(), -rotation.cos()) / 10.;
            self.last_shot = frame_t;
        }

        // Euler integration
        self.vel += acc;
        self.pos += self.vel;
        self.pos = wrap_around(&self.pos);
    }

    fn draw(&self) {
        let rotation = self.rot.to_radians();

        let v1 = Vec2::new(
            self.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
            self.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.pos.x
                - rotation.cos() * SHIP_BASE / 2.
                - rotation.sin() * SHIP_HEIGHT / 2.,
            self.pos.y - rotation.sin() * SHIP_BASE / 2.
                + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.pos.x + rotation.cos() * SHIP_BASE / 2.
                - rotation.sin() * SHIP_HEIGHT / 2.,
            self.pos.y
                + rotation.sin() * SHIP_BASE / 2.
                + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., self.color);
    }
}

#[derive(Default)]
struct ShipControl {
    thrust_forward: bool,
    thrust_aft: bool,
    thrust_left: bool,
    thrust_right: bool,
    rotate_left: bool,
    rotate_right: bool,
    fire: bool,
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
            sides: rand::gen_range(6, 12),
            collided: false,
        }
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
    let mut game = Game::new(0);

    loop {
        clear_background(BLACK);

        if game.game_over {
            let win = game.asteroids.is_empty();
            game = wait_for_new_game(win).await;
        }
        let frame_t = get_time();

        let ship_control = ShipControl {
            thrust_forward: is_key_down(KeyCode::D) || is_key_down(KeyCode::Up),
            thrust_aft: is_key_down(KeyCode::E) || is_key_down(KeyCode::Down),
            thrust_left: is_key_down(KeyCode::F),
            thrust_right: is_key_down(KeyCode::S),
            rotate_left: is_key_down(KeyCode::Left),
            rotate_right: is_key_down(KeyCode::Right),
            fire: is_key_down(KeyCode::Space),
        };
        game.ship.update(ship_control, &mut game.bullets);

        let enemy_angle = game.enemy.rot.to_radians();
        let enemy_dir = Vec2::from_angle(enemy_angle);
        let to_ship = game.enemy.pos - game.ship.pos;
        let target_angle_rad = enemy_dir.angle_between(to_ship);
        let degrees = target_angle_rad.to_degrees() + 90.;
        dbg!(degrees);
        let modulo = degrees.rem_euclid(360.);
        let centered = modulo - 180.0;
        dbg!(centered);

        let enemy_control = ShipControl {
            rotate_left: centered < 0.,
            rotate_right: centered > 0.,
            thrust_forward: centered.abs() < 45.,
            thrust_aft: centered.abs() > 90.,
            thrust_left: centered < 0.,
            thrust_right: centered > 0.,
            fire: centered.abs() < 5.,
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
        game.bullets.retain(|bullet| bullet.shot_at + 2.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in game.asteroids.iter_mut() {
            // ship collision
            if (asteroid.pos - game.ship.pos).length()
                < asteroid.size + SHIP_HEIGHT / 3.
            {
                // game.game_over = true;
                break;
            }

            // Asteroid/bullet collision
            for bullet in game.bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;

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
        game.ship.draw();
        game.enemy.draw();
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
