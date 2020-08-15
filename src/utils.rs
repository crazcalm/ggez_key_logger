use ggez::graphics;

use rand::Rng;

pub fn random_color() -> graphics::Color {
    let mut rng = rand::thread_rng();

    let r: f32 = rng.gen();
    let g: f32 = rng.gen();
    let b: f32 = rng.gen();
    let a: f32 = 1.0;

    graphics::Color::new(r, g, b, a)
}
