#![warn(
    clippy::cognitive_complexity,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::large_digit_groups
)]
mod bullet;
mod game;
mod particle;
mod screen;
mod ship;

use game::Game;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use screen::wrap_around;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game = wait_for_new_game(false).await;
    let mut skin = root_ui().default_skin();
    let style = root_ui()
        .style_builder()
        .color(BLACK)
        .text_color(WHITE)
        .build();
    skin.label_style = style;
    root_ui().push_skin(&skin);

    loop {
        clear_background(BLACK);
        let mut new_objects: Vec<_> =
            game.objects.iter_mut().flat_map(|o| o.update()).collect();
        game.objects.retain(|o| o.is_alive());
        game.objects.append(&mut new_objects);
        game.objects.iter().for_each(|o| o.draw());
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
            return Game::new();
        }
        next_frame().await;
        continue;
    }
}

fn show_win() {
    show_game_over("You Win!. Press [enter] to play again.");
}

fn show_loss() {
    show_game_over("Press [enter] to play.");
}

fn show_game_over(text: &str) {
    let font_size = 30.;
    let text_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen::center().x - text_size.width / 2.,
        screen::center().y - text_size.height / 2.,
        font_size,
        LIGHTGRAY,
    );
}
