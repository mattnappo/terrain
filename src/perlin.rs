use ggez::{
    self, event,
    graphics::{self, Color},
    {Context, GameResult},
};
use glam::*;
use rand::{self, rngs::StdRng, Rng, SeedableRng};
use std::{f32::consts, time::SystemTime};

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

// Helper method to interpolate between two values.
fn interpolate(a: f32, b: f32, w: f32) -> f32 {
    // check that w \in [0, 1]
    (a - b) * (3.0 - w * 2.0) * w * w + a
}

/// The game's main state
struct State {
    /// Number of columns (of vectors in x dir)
    x: usize,

    /// Number of rows (of vectors in y dir)
    y: usize,

    /// The initial random unit gradient vectors (unit = 1.0)
    /// Size: (y, x) (rows, cols)
    vecs: Vec<Vec<Vec2>>,

    /// The final, interpolated noise. Each f32 is a value in the range [0, 1],
    /// representing the color value of a pixel at the point (x, y).
    /// Size: (y*scale, x*scale) (rows, cols) (in pixels)
    noise: Vec<Vec<f32>>,

    /// All game randomness comes from this rng
    prng: StdRng,

    // The rng's seed
    #[allow(dead_code)]
    seed: u64,

    /* -- For rendering only -- */
    /// Whether the scene was drawn already
    drawn: bool,

    /// The number of pixels between each grid, the length of the arrows
    /// when drawn (in pixels)
    scale: f32,

    /// The number of pixels in the x direction (disregarding the shift)
    x_pixels: usize,

    /// The number of pixels in the y direction (disregard the shift)
    y_pixels: usize,

    /// Linear shift, in pixels (for rendering only)
    shift: f32,
}

impl State {
    /// Create a new game state
    fn new(
        x: usize,
        y: usize,
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
            vecs: Vec::new(),
            noise: Vec::new(),
            prng,
            seed: use_seed,
            drawn: false,
            scale,
            x_pixels: x * scale as usize,
            y_pixels: y * scale as usize,
            shift,
        };
        state.gen_vecs();
        state.calc_perlin();
        Ok(state)
    }

    /// Generate a random vector of magnitude 1.0 with this state's prng
    fn random_unit_vector(&mut self) -> Vec2 {
        let a = self.prng.gen_range(0.0, PI2);
        Vec2::new(a.cos(), a.sin())
    }

    /// Generate the random vectors
    fn gen_vecs(&mut self) {
        // Step 1
        // Assign a random vector to each point in the grid
        for x in 0..self.x {
            let mut row: Vec<Vec2> = Vec::new();
            for y in 0..self.y {
                row.push(self.random_unit_vector());
            }
            self.vecs.push(row);
        }
    }

    /// Calculate the dot products and interpolation
    fn calc_perlin(&mut self) {
        // For every single pixel:
        // Find which grid that pixel lies in (top left origin)
        // Calculate the displacement vectors from that pixel to each of the 4 corners
        // Dot each displacement vector with its associated corner gradient vector
        // Interpolate the x dots
        // Interpolate the y dots
        // Interpolate the interpolation of the x dots and the interpolation of the y dots
        // Other todo: Check that the interpolation function is all good
    }

    /* -- Drawing functions -- */

    /// Draw the lattice grid
    fn draw_grid(&self, ctx: &mut Context) -> GameResult {
        let bs = self.scale as f32;
        for x in 0..self.x {
            let xf = x as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(xf * bs + self.shift, self.shift),
                    Vec2::new(
                        xf * bs + self.shift,
                        bs * (self.y - 1) as f32 + self.shift,
                    ),
                ],
                1.0,
                RED,
            )?;
            graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
        }

        for y in 0..self.y {
            let yf = y as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(self.shift, yf * bs + self.shift),
                    Vec2::new(
                        self.shift + bs * (self.x - 1) as f32,
                        yf * bs + self.shift,
                    ),
                ],
                1.0,
                RED,
            )?;
            graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
        }
        Ok(())
    }

    /// Draw a 2D arrow/vector
    fn draw_vector(
        &self,
        ctx: &mut Context,
        v: Vec2,
        pos: (f32, f32),
    ) -> GameResult {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(pos.0, pos.1),
                Vec2::new(pos.0 + v.x * self.scale, pos.1 + v.y * self.scale),
            ],
            1.0,
            BLACK,
        )?;
        let head_length = self.scale / 5.0;
        let vec_angle = v.y.atan2(v.x);
        let rhead = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(
                    (pos.0 + v.x * self.scale)
                        - head_length * (vec_angle + consts::FRAC_PI_4).sin(),
                    (pos.1 + v.y * self.scale)
                        + head_length * (vec_angle + consts::FRAC_PI_4).cos(),
                ),
                Vec2::new(pos.0 + v.x * self.scale, pos.1 + v.y * self.scale),
            ],
            1.0,
            BLACK,
        )?;
        let lhead = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(
                    (pos.0 + v.x * self.scale)
                        + head_length * (vec_angle - consts::FRAC_PI_4).sin(),
                    (pos.1 + v.y * self.scale)
                        - head_length * (vec_angle - consts::FRAC_PI_4).cos(),
                ),
                Vec2::new(pos.0 + v.x * self.scale, pos.1 + v.y * self.scale),
            ],
            1.0,
            BLACK,
        )?;
        graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &rhead, (Vec2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &lhead, (Vec2::new(0.0, 0.0),))?;

        Ok(())
    }

    /// Draw all the random vectors on the grid
    fn draw_vectors(&self, ctx: &mut Context) -> GameResult {
        for x in 0..self.x {
            for y in 0..self.y {
                self.draw_vector(
                    ctx,
                    self.vecs[y][x],
                    (
                        x as f32 * self.scale + self.shift,
                        y as f32 * self.scale + self.shift,
                    ),
                )?;
            }
        }
        Ok(())
    }

    /// Draw the noise
    fn draw_noise(&self, ctx: &mut Context) -> GameResult {
        for x in 0..self.x_pixels {
            for y in 0..self.y_pixels {
                // PLEASE FIND A MORE EFFICIENT WAY TO DO THIS
                let px = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        x as f32 + self.shift,
                        y as f32 + self.shift,
                        1.0,
                        1.0,
                    ),
                    Color::new(0.0, 0.0, self.noise[y][x], 1.0),
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
        self.draw_noise(ctx)?;

        self.drawn = true;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("terrain generation", "xoreo")
        .window_setup(ggez::conf::WindowSetup {
            title: "terrain generation".to_owned(),
            samples: ggez::conf::NumSamples::One,
            vsync: false,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(ggez::conf::WindowMode {
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
        });
    let (mut ctx, mut event_loop) = cb.build()?;

    let mut state = State::new(
        3,     // Number of cols
        3,     // Number of rows
        100.0, // Scale
        100.0, // Rendering shift
        None,  // Seed
    )?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
