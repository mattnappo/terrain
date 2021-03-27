use ggez;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use std::env;
use std::path;

struct State {
    map: Vec<Vec<Vec2>>,
}

impl State {
    fn new(x: usize, y: usize) -> GameResult<Self> {
        // Build the lattice map
        Ok(Self {
            map: vec![vec![Vec2::new()]],
        })
    }
}

fn main() {
    println!("Hello, world!");
}
