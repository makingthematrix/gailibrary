//! The simplest possible example that does something.
extern crate ggez;

use ggez::{conf, event, Context};

use gailibrary::examples::game_of_life::GameOfLife;
use gailibrary::fields::Pos2D;
use gailibrary::visualisation::*;

pub fn main() {
    let window_size: usize = 800;
    let dim = 100;

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("game_of_life", "makingthematrix", c).unwrap();
    setup(ctx, window_size);

    let state = &mut MainState::<GameOfLife>::new(window_size, dim).unwrap();
    state.add(&Pos2D::new((dim as i64) / 2, (dim as i64) / 2));
    event::run(ctx, state).unwrap();
}
