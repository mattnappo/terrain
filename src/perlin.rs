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
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

// Helper method to interpolate between two values with the given weight.
fn interpolate(a: f32, b: f32, w: f32) -> f32 {
    (a - b) * (3.0 - w * 2.0) * w * w + a
}

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
            x: x - 1,
            y: y - 1,
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
        state.calc_perlin();
        Ok(state)
    }

    fn random_unit_vector(&mut self) -> Vec2 {
        let a = self.prng.gen_range(0.0, PI2);
        Vec2::new(self.scale * a.cos(), self.scale * a.sin())
    }

    fn gen_vecs(&mut self) {
        // Step 1
        // Assign a random vector to each point in the grid
        for x in 0..=self.x {
            let mut row: Vec<Vec2> = Vec::new();
            for y in 0..=self.y {
                row.push(self.random_unit_vector());
            }
            self.vecs.push(row);
        }
    }

    fn calc_perlin(&mut self) {
        // Step 2 and 3. Calc dot products and interpolate.
        for x in 0..((self.box_size * (self.x - 0) as f32) as usize) {
            let mut row: Vec<f32> = Vec::new();
            for y in 0..((self.box_size * (self.y - 0) as f32) as usize) {
                //let gridx = (x as f32 % (self.box_size / self.x as f32)) as usize;
                //let gridy = (y as f32 % (self.box_size / self.y as f32)) as usize;
                let gridx = (x / self.box_size as usize) % self.x;
                let gridy = (y / self.box_size as usize) % self.y;
                let gridxf = gridx as f32;
                let gridyf = gridy as f32;

                let off_ul = Vec2::new((x - gridx) as f32, (y - gridy) as f32);
                let off_ur = Vec2::new(x - (gridx as f32 + self.box_size), y - gridy);
                let off_bl = Vec2::new(x - gridx, y - (gridy + self.box_size));
                let off_br = Vec2::new(x - gridx, y - gridy);

                println!("real: (x,y) = ({}, {})", x, y);
                println!("grid: (x,y) = ({}, {})", gridx, gridy);
                println!(" off: (x,y) = ({}, {})\n", x - gridx, y - gridy);
                //println!("x, y = {}, {} --> ({}, {})", x, y, gridx, gridy);
                // let gridy: usize = y % (self.y as usize);
                //println!("{} {}", gridx, gridy);

                let dot1 =
                    Vec2::new((x - gridx) as f32, (y - gridy) as f32).dot(self.vecs[gridy][gridx]);
                let dot2 = Vec2::new((x - gridx + 1) as f32, (y - gridy) as f32)
                    .dot(self.vecs[gridy][gridx + 1]);
                let dot3 = Vec2::new((x - gridx) as f32, (y - gridy + 1) as f32)
                    .dot(self.vecs[gridy + 1][gridx]);
                let dot4 = Vec2::new((x - gridx + 1) as f32, (y - gridy + 1) as f32)
                    .dot(self.vecs[gridy + 1][gridx + 1]);

                // Temp
                let i1 = interpolate(dot1, dot2, (x - gridx) as f32);
                let i2 = interpolate(dot3, dot4, (x - gridx) as f32);
                let i = interpolate(i1, i2, (y - gridy) as f32);

                row.push(i);
            }
            self.map.push(row);
        }
    }

    fn draw_grid(&self, ctx: &mut Context) -> GameResult {
        let bs = self.box_size as f32;
        for x in 0..=self.x {
            let xf = x as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(xf * bs + self.shift, self.shift),
                    Vec2::new(xf * bs + self.shift, bs * self.y as f32 + self.shift),
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
                    Vec2::new(self.shift, yf * bs + self.shift),
                    Vec2::new(self.shift + bs * self.x as f32, yf * bs + self.shift),
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
        let vec_angle = v.y.atan2(v.x);
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
        for x in 0..=self.x {
            for y in 0..=self.y {
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

    fn draw_map(&self, ctx: &mut Context) -> GameResult {
        for x in 0..(self.box_size * self.x as f32) as usize {
            for y in 0..(self.box_size * self.y as f32) as usize {
                let b = self.map[y][x] / (self.x as f32 * self.box_size * self.scale);
                //println!("{}", b);
                let px = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(x as f32 + self.shift, y as f32 + self.shift, 1.0, 1.0),
                    Color::new(0.0, 0.0, b, 1.0),
                )?;
                graphics::draw(ctx, &px, (Vec2::new(0.0, 0.0),))?;
            }
        }
        Ok(())
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, WHITE);

        if self.drawn {
            return Ok(());
        }

        self.draw_grid(ctx)?;
        self.draw_vectors(ctx)?;
        self.draw_map(ctx)?;

        self.drawn = true;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let w_setup = ggez::conf::WindowSetup {
        title: "terrain generation".to_owned(),
        samples: ggez::conf::NumSamples::One,
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
    };
    let w_mode = ggez::conf::WindowMode {
        width: 600.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    };

    let cb = ggez::ContextBuilder::new("terrain generation", "xoreo")
        .window_setup(w_setup)
        .window_mode(w_mode);
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut state = State::new(
        3,    // Number of cols
        3,    // Number of rows
        10.0, // Box size
        10.0, // Scale
        //40.0,  // Vector scale
        5.0,  // Rendering shift
        None, // Seed
    )?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
