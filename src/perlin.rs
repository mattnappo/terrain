use ggez::{
    self, event,
    graphics::{self, Color},
    {Context, GameResult},
};
use glam::*;
use rand::{self, rngs::StdRng, Rng, SeedableRng};
use std::{env, f32::consts, path, time::SystemTime};

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

/// Helper method to interpolate between two values.
fn interpolate(a: f32, b: f32, w: f32) -> f32 {
    // check that w \in [0, 1]
    println!("w: {}", w);
    (a - b) * (3.0 - w * 2.0) * w * w + a
}

/// Helper method to format a vector
fn format_vec(v: &Vec2) -> String {
    let angle = v.y.atan2(v.x);
    let mag = (v.x * v.x + v.y * v.y).sqrt();
    format!("Vec {{ x:{}, y:{}, a:{}, m:{} }}", v.x, v.y, angle, mag)
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
    xpx: usize,

    /// The number of pixels in the y direction (disregard the shift)
    ypx: usize,

    /// Linear shift, in pixels (for rendering only)
    shift: f32,

    /// The font for drawing text
    font: graphics::Font,

    /// Debugging stuff
    debug_vecs: Vec<Vec2>,
    frames: u32,
}

impl State {
    /// Create a new game state
    fn new(
        x: usize,
        y: usize,
        scale: f32,
        shift: f32,
        seed: Option<u64>,
        ctx: &mut Context,
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
        let prng = StdRng::seed_from_u64(use_seed);

        let mut state = Self {
            x,
            y,
            vecs: Vec::new(),
            noise: vec![vec![0.0; x * scale as usize]; y * scale as usize],
            prng,
            seed: use_seed,
            drawn: false,
            scale,
            xpx: x * scale as usize,
            ypx: y * scale as usize,
            shift,
            font: graphics::Font::new(ctx, "/Roboto-Medium.ttf")?,
            debug_vecs: vec![Vec2::new(0.0, 0.0); 4],
            frames: 0,
        };
        state.gen_vecs();
        //state.calc_perlin();
        Ok(state)
    }

    /// Generate a random vector of magnitude 1.0 with this state's prng
    fn random_unit_vector(&mut self) -> Vec2 {
        let a = self.prng.gen_range(0.0, PI2);
        Vec2::new(a.cos(), a.sin())
    }

    /// Generate the random vectors
    fn gen_vecs(&mut self) {
        // Assign a random vector to each point in the grid
        for _ in 0..self.x {
            let mut row: Vec<Vec2> = Vec::new();
            for _ in 0..self.y {
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

        for x in 0..self.xpx {
            for y in 0..self.ypx {
                let pos = Vec2::new(x as f32 - self.shift, y as f32 - self.shift);
                let gridx = (pos.x / self.scale) as usize;
                let gridy = (pos.y / self.scale) as usize;
                let gridxf = gridx as f32;
                let gridyf = gridy as f32;

                // Offset vectors
                let tlo = Vec2::new(pos.x - gridxf, pos.y - gridyf);
                let tro = Vec2::new(pos.x - (gridxf + self.scale), pos.y - gridyf);
                let blo = Vec2::new(pos.x - gridxf, pos.y - (gridyf + self.scale));
                let bro = Vec2::new(
                    pos.x - (gridxf + self.scale),
                    pos.y - (gridyf + self.scale),
                );

                // Corner vectors
                let tlv = self.vecs[gridy][gridx];
                let trv = self.vecs[gridy][gridx + 1];
                let blv = self.vecs[gridy + 1][gridx];
                let brv = self.vecs[gridy + 1][gridx + 1];

                // Dot products
                let d1 = tlo.dot(tlv);
                let d2 = tro.dot(trv);
                let d3 = blo.dot(blv);
                let d4 = bro.dot(brv);
                println!("(d1, d2, d3, d4) = ({}, {}, {}, {})", d1, d2, d3, d4);

                // Interpolation
                //let i1 = interpolate(d1, d2);
                //let i1 = interpolate(d3, d4);
                //let i = interpolate(d1, d2);
            }
        }
    }

    fn calc_perlin_debug(&mut self, x: f32, y: f32) -> f32 {
        let pos = Vec2::new(x as f32 - self.shift, y as f32 - self.shift);
        let gridx = (pos.x / self.scale) as usize;
        let gridy = (pos.y / self.scale) as usize;
        let gridxf = gridx as f32;
        let gridyf = gridy as f32;

        // Offset vectors
        let tlo =
            Vec2::new(pos.x - gridxf * self.scale, pos.y - gridyf * self.scale);
        let tro = Vec2::new(
            pos.x - (gridxf + 1.0) * self.scale,
            pos.y - gridyf * self.scale,
        );
        let blo = Vec2::new(
            pos.x - gridxf * self.scale,
            pos.y - (gridyf + 1.0) * self.scale,
        );
        let bro = Vec2::new(
            pos.x - (gridxf + 1.0) * self.scale,
            pos.y - (gridyf + 1.0) * self.scale,
        );

        self.debug_vecs[0] = tlo;
        self.debug_vecs[1] = tro;
        self.debug_vecs[2] = blo;
        self.debug_vecs[3] = bro;

        // Corner vectors
        let tlv = self.vecs[gridy][gridx];
        let trv = self.vecs[gridy][gridx + 1];
        let blv = self.vecs[gridy + 1][gridx];
        let brv = self.vecs[gridy + 1][gridx + 1];

        // Dot products
        let d1 = tlo.dot(tlv);
        let d2 = tro.dot(trv);
        let d3 = blo.dot(blv);
        let d4 = bro.dot(brv);
        //println!("(d1, d2, d3, d4) = ({}, {}, {}, {})", d1, d2, d3, d4);

        // Interpolation
        let i1 = interpolate(d1, d2, tlo.x / self.scale);
        let i2 = interpolate(d3, d4, tlo.x / self.scale);
        let i = interpolate(i1, i2, tlo.y / self.scale);
        self.noise[y as usize][x as usize] = i;
        i
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
            &[Vec2::new(pos.0, pos.1), Vec2::new(pos.0 + v.x, pos.1 + v.y)],
            1.0,
            BLACK,
        )?;
        let head_length = self.scale / 8.0;
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
        for x in 0..self.xpx {
            for y in 0..self.ypx {
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

    /// Helper method to generate a new piece of text, ready for rendering
    fn draw_text(
        &self,
        ctx: &mut Context,
        text: String,
        pos: (f32, f32),
    ) -> GameResult {
        let text = graphics::Text::new(graphics::TextFragment {
            text,
            color: Some(BLACK),
            font: Some(self.font),
            scale: Some(graphics::Scale { x: 24.0, y: 24.0 }),
        });
        graphics::draw(ctx, &text, (Vec2::new(pos.0, pos.1),))?;
        Ok(())
    }

    /// Draw debug information
    fn draw_debug(&mut self, ctx: &mut Context) -> GameResult {
        // Draw the grid space and mouse position
        let pos = ggez::input::mouse::position(ctx);
        let gridx = ((pos.x - self.shift) / self.scale) as usize as f32;
        let gridy = ((pos.y - self.shift) / self.scale) as usize as f32;

        self.draw_text(ctx, format!("({},{})", pos.x, pos.y), (0.0, 0.0))?;
        self.draw_text(ctx, format!("({},{})", gridx, gridy), (0.0, 50.0))?;

        for (i, vec) in self.debug_vecs.iter().enumerate() {
            self.draw_text(ctx, format_vec(vec), (0.0, 25.0 * i as f32 + 400.0))?;
        }

        // Draw the debug offset vectors and the perlin noise value at the cursor
        let noise = self.calc_perlin_debug(pos.x, pos.y);
        self.draw_text(ctx, noise.to_string(), (0.0, 550.0))?;

        self.draw_vector(
            ctx,
            self.debug_vecs[0],
            (
                gridx * self.scale + self.shift,
                gridy * self.scale + self.shift,
            ),
        )?;
        self.draw_vector(
            ctx,
            self.debug_vecs[1],
            (
                (gridx + 1.0) * self.scale + self.shift,
                gridy * self.scale + self.shift,
            ),
        )?;
        self.draw_vector(
            ctx,
            self.debug_vecs[2],
            (
                gridx * self.scale + self.shift,
                (gridy + 1.0) * self.scale + self.shift,
            ),
        )?;
        self.draw_vector(
            ctx,
            self.debug_vecs[3],
            (
                (gridx + 1.0) * self.scale + self.shift,
                (gridy + 1.0) * self.scale + self.shift,
            ),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, WHITE);

        self.draw_debug(ctx)?;

        if self.drawn {
            return Ok(());
        }

        self.draw_grid(ctx)?;
        self.draw_vectors(ctx)?;
        //self.draw_noise(ctx)?;

        self.frames += 1;
        // println!("frame {}", self.frames);

        //self.drawn = true;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let res_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };

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
            resizable: false,
        })
        .add_resource_path(res_dir);
    let (mut ctx, mut event_loop) = cb.build()?;

    let mut state = State::new(
        3,     // Number of cols
        3,     // Number of rows
        100.0, // Scale
        100.0, // Rendering shift
        None,  // Seed
        // Some(1617081672u64),
        &mut ctx,
    )?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
