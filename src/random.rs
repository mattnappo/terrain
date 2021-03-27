use ggez;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use rand;
use rand::Rng;
use rand::SeedableRng;
use std::time::SystemTime;

const BLACK: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
const WHITE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

struct State {
    x: usize,
    y: usize,
    map: Vec<Vec<f32>>,
    prng: rand::rngs::StdRng,
    seed: u64,
    box_size: f32,
    drawn: bool,
}

impl State {
    fn new(x: usize, y: usize, box_size: f32, seed: Option<u64>) -> GameResult<Self> {
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

        Ok(Self {
            x,
            y,
            map: vec![vec![prng.gen::<f32>(); x]; y],
            prng,
            seed: use_seed,
            box_size,
            drawn: false,
        })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());

        if self.drawn {
            return Ok(());
        }

        for x in 0..self.x {
            for y in 0..self.y {
                let mut color = BLACK;
                if self.prng.gen::<u8>() > 128 {
                    color = WHITE;
                }
                let square = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        x as f32 * self.box_size,
                        y as f32 * self.box_size,
                        self.box_size,
                        self.box_size,
                    ),
                    color,
                )?;
                graphics::draw(ctx, &square, (Vec2::new(0.0, 0.0),))?;
            }
        }
        self.drawn = true;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("terrain generation", "xoreo");
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut state = State::new(100, 100, 10.0, None)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
