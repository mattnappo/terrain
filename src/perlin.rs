use ggez;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use rand;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::f32::consts;
use std::time::SystemTime;

const PI2: f32 = 2.0 * consts::PI;

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
    vecs: Vec<Vec<Vec2>>, // The random vecs
    map: Vec<Vec<f32>>,   // The final interpolated map
    prng: StdRng,
    seed: u64,
    box_size: f32,
    drawn: bool,
    scale: f32,
    shift: f32,
}

impl State {
    fn new(
        x: usize,
        y: usize,
        box_size: f32,
        scale: f32,
        shift: f32,
        seed: Option<u64>,
    ) -> GameResult<Self> {
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
        let mut prng = StdRng::seed_from_u64(use_seed);

        let mut state = Self {
            x,
            y,
            map: Vec::new(),
            vecs: Vec::new(),
            prng,
            seed: use_seed,
            box_size,
            drawn: false,
            scale,
            shift,
        };
        state.gen_vecs();
        Ok(state)
    }

    fn random_unit_vector(&mut self) -> Vec2 {
        let a = self.prng.gen_range(0.0, PI2);
        Vec2::new(self.scale * a.cos(), self.scale * a.sin())
    }

    fn gen_vecs(&mut self) {
        // Assign a random vector to each point in the grid
        for x in 0..self.x {
            let mut row: Vec<Vec2> = Vec::new();
            for y in 0..self.y {
                row.push(self.random_unit_vector());
            }
            self.vecs.push(row);
        }
    }

    fn draw_grid(&self, ctx: &mut Context) -> GameResult {
        let bs = self.box_size as f32;
        for x in 0..=self.x {
            let xf = x as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(xf * bs, 0.0),
                    Vec2::new(xf * bs, bs * self.y as f32),
                ],
                1.0,
                RED,
            )?;
            graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
        }

        for y in 0..=self.y {
            let yf = y as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0.0, yf * bs),
                    Vec2::new(bs * self.x as f32, yf * bs),
                ],
                1.0,
                RED,
            )?;
            graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
        }
        Ok(())
    }

    fn draw_vector(&self, ctx: &mut Context, v: Vec2, pos: (f32, f32)) -> GameResult {
        let line = graphics::Mesh::new_line(
            ctx,
            &[Vec2::new(pos.0, pos.1), Vec2::new(pos.0 + v.x, pos.1 + v.y)],
            1.0,
            BLACK,
        )?;
        let head_length = self.scale / 5.0;
        let vec_angle = (v.y / v.x).atan();
        let rhead = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(
                    (pos.0 + v.x) - head_length * (vec_angle + consts::FRAC_PI_4).sin(),
                    (pos.1 + v.y) + head_length * (vec_angle + consts::FRAC_PI_4).cos(),
                ),
                Vec2::new(pos.0 + v.x, pos.1 + v.y),
            ],
            1.0,
            BLACK,
        )?;
        let lhead = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(
                    (pos.0 + v.x) + head_length * (vec_angle - consts::FRAC_PI_4).sin(),
                    (pos.1 + v.y) - head_length * (vec_angle - consts::FRAC_PI_4).cos(),
                ),
                Vec2::new(pos.0 + v.x, pos.1 + v.y),
            ],
            1.0,
            BLACK,
        )?;
        graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &rhead, (Vec2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &lhead, (Vec2::new(0.0, 0.0),))?;

        Ok(())
    }

    fn draw_vectors(&self, ctx: &mut Context) -> GameResult {
        for x in 0..self.x {
            for y in 0..self.y {
                self.draw_vector(
                    ctx,
                    self.vecs[y][x],
                    (
                        x as f32 * self.box_size + self.shift,
                        y as f32 * self.box_size + self.shift,
                    ),
                )?;
            }
        }
        Ok(())
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());

        if self.drawn {
            return Ok(());
        }

        self.draw_grid(ctx)?;
        self.draw_vectors(ctx)?;

        let color = WHITE;
        self.drawn = true;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("terrain generation", "xoreo");
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut state = State::new(
        5,     // Number of cols
        5,     // Number of rows
        100.0, // Box size
        55.0,  // Vector scale
        50.0,  // Rendering shift
        None,  // Seed
    )?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
