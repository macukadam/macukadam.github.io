pub mod manager {
    use crate::entities::entities::{wrap_around, Asteroid, Bullet, Ship, SHIP_HEIGHT};
    use macroquad::prelude::*;

    pub async fn check_game_status(
        bullets: &Vec<Bullet>,
        asteroids: &Vec<Asteroid>,
        ship: &Ship,
    ) -> GameState {
        if asteroids.len() == 0 {
            return GameState::Over;
        }

        clear_background(LIGHTGRAY);
        for bullet in bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., BLACK);
        }

        for asteroid in asteroids.iter() {
            draw_poly_lines(
                asteroid.pos.x,
                asteroid.pos.y,
                asteroid.sides,
                asteroid.size,
                asteroid.rot,
                2.,
                RED,
            );

            if (asteroid.pos - ship.pos).length() < asteroid.size + SHIP_HEIGHT / 3. {
                return GameState::Over;
            }
        }

        let (v1, v2, v3) = ship.forward();
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
        next_frame().await;
        GameState::Continue
    }

    pub async fn new_game() -> (Vec<Bullet>, Vec<Asteroid>, f64, Ship) {
        let bullets = Vec::new();
        let last_shot = get_time();
        let mut asteroids = Vec::new();

        for _ in 0..100 {
            asteroids.push(Asteroid::new())
        }
        let ship = Ship::new();
        return (bullets, asteroids, last_shot, ship);
    }

    pub async fn show_menu(asteroids: &Vec<Asteroid>) {
        clear_background(LIGHTGRAY);
        let mut text = "You Win!. Press [enter] to play again.";
        let font_size = 30.;

        if asteroids.len() > 0 {
            text = "Game Over. Press [enter] to play again.";
        }
        let text_size = measure_text(text, None, font_size as _, 1.0);
        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. - text_size.height / 2.,
            font_size,
            DARKGRAY,
        );
    }

    pub async fn game_logic(
        mut bullets: Vec<Bullet>,
        mut asteroids: Vec<Asteroid>,
        mut last_shot: f64,
        mut ship: Ship,
    ) -> (Vec<Bullet>, Vec<Asteroid>, f64, Ship) {
        let frame_t = get_time();
        let rotation = ship.rot.to_radians();

        let mut acc = -ship.vel / 10.;
        if is_key_down(KeyCode::Up) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }

        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.1 {
            bullets.push(Bullet::new(ship.pos.clone(), rotation, frame_t));
            last_shot = frame_t;
        }
        if is_key_down(KeyCode::Right) {
            ship.rot += 5.;
        } else if is_key_down(KeyCode::Left) {
            ship.rot -= 5.;
        }

        ship.vel += acc;
        if ship.vel.length() > 5. {
            ship.vel = ship.vel.normalize() * 5.;
        }
        ship.pos += ship.vel;
        ship.pos = wrap_around(ship.pos);
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }
        for asteroid in asteroids.iter_mut() {
            asteroid.pos += asteroid.vel;
            asteroid.pos = wrap_around(asteroid.pos);
            asteroid.rot += asteroid.rot_speed;
        }

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            if (asteroid.pos - ship.pos).length() < asteroid.size + SHIP_HEIGHT / 3. {
                break;
            }
            for bullet in bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;
                    if asteroid.sides > 4 {
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(bullet.vel.y, -bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(-bullet.vel.y, bullet.vel.x).normalize()
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

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
        asteroids.retain(|asteroid| !asteroid.collided);
        asteroids.append(&mut new_asteroids);

        (bullets, asteroids, last_shot, ship)
    }

    pub async fn show_menu_logic(
        asteroids: &Vec<Asteroid>,
    ) -> Option<(Vec<Bullet>, Vec<Asteroid>, f64, Ship)> {
        show_menu(&asteroids).await;
        if is_key_down(KeyCode::Enter) {
            return Some(new_game().await);
        }
        next_frame().await;
        None
    }
    pub enum GameState {
        Over,
        Continue,
    }
}
