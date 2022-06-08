extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use manager::manager::*;

mod entities;
mod manager;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "This is a Game".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    window_conf();
    let (mut bullets, mut asteroids, mut last_shot, mut ship) = new_game().await;

    loop {
        let game_state = check_game_status(&bullets, &asteroids, &ship).await;

        if let GameState::Over = game_state {
            match show_menu_logic(&asteroids).await {
                Some((b, a, l, s)) => {
                    bullets = b;
                    asteroids = a;
                    last_shot = l;
                    ship = s;
                }
                None => {
                    continue;
                }
            }
        } else {
            (bullets, asteroids, last_shot, ship) =
                game_logic(bullets, asteroids, last_shot, ship).await;
        }
    }
}
