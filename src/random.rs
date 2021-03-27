use ggez;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use mint;
use rand;
use rand::Rng;
use rand::SeedableRng;
use std::env;
use std::mem::transmute;
use std::path;
use std::time::SystemTime;

struct State {
    map: Vec<Vec<f32>>,
    prng: rand::rngs::StdRng,
    seed: u64,
}

impl State {
    fn new(x: usize, y: usize, seed: Option<u64>) -> GameResult<Self> {
        // Build the prng
        let mut use_seed: u64 = 0;
        if let Some(s) = seed {
            use_seed = s;
        } else {
            if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                use_seed = n.as_secs();
            }
        }
        println!("seed is: {}", use_seed);
        let mut prng = rand::rngs::StdRng::seed_from_u64(use_seed);

        // Build the lattice map
        Ok(Self {
            map: vec![vec![prng.gen::<f32>(); x]; y],
            prng,
            seed: use_seed,
        })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());

        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(10.0, 10.0, 10.0, 10.0),
            Color::new(10.0, 10.0, 10.0, 10.0),
        )?;

        graphics::draw(ctx, &square, (Vec2::new(10.0, 38.0),))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
