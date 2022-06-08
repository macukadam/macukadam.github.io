pub mod entities {
    use macroquad::prelude::*;
    pub const SHIP_HEIGHT: f32 = 25.;
    pub const SHIP_BASE: f32 = 22.;

    pub struct Ship {
        pub pos: Vec2,
        pub rot: f32,
        pub vel: Vec2,
    }

    impl Ship {
        pub fn new() -> Self {
            Self {
                pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
                rot: 0.,
                vel: Vec2::new(0., 0.),
            }
        }

        pub fn forward(self: &Self) -> (Vec2, Vec2, Vec2) {
            let rotation = self.rot.to_radians();
            let v1 = Vec2::new(
                self.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
                self.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
            );
            let v2 = Vec2::new(
                self.pos.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
                self.pos.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
            );
            let v3 = Vec2::new(
                self.pos.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
                self.pos.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
            );

            (v1, v2, v3)
        }
    }

    pub struct Bullet {
        pub pos: Vec2,
        pub vel: Vec2,
        pub shot_at: f64,
        pub collided: bool,
    }

    impl Bullet {
        pub fn new(ship_pos: Vec2, rotation: f32, frame_t: f64) -> Self {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            Self {
                pos: ship_pos + rot_vec * SHIP_HEIGHT / 2.,
                vel: rot_vec * 7.,
                shot_at: frame_t,
                collided: false,
            }
        }
    }

    pub struct Asteroid {
        pub pos: Vec2,
        pub vel: Vec2,
        pub rot: f32,
        pub rot_speed: f32,
        pub size: f32,
        pub sides: u8,
        pub collided: bool,
    }

    impl Asteroid {
        pub fn new() -> Self {
            let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
            Self {
                pos: screen_center
                    + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                        * screen_width().min(screen_height())
                        / 2.,
                vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
                rot: 0.,
                rot_speed: rand::gen_range(-2., 2.),
                size: screen_width().min(screen_height()) / 10.,
                sides: 6,
                collided: false,
            }
        }
    }

    pub fn wrap_around(v: Vec2) -> Vec2 {
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
}
