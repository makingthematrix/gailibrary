//! The simplest possible example that does something.
extern crate ggez;

use ggez::{conf, event, Context};

use gailibrary::examples::langtons_ant::LangtonsAnt;
use gailibrary::fields::Pos2D;
use gailibrary::visualisation::*;

pub fn main() {
    let window_size: usize = 800;
    let dim = 100;

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("langtons_ant", "makingthematrix", c).unwrap();
    setup(ctx, window_size);

    let state = &mut MainState::<LangtonsAnt>::new(window_size, dim).unwrap();
    state.add(&Pos2D::new((dim as i64) / 2, (dim as i64) / 2));
    event::run(ctx, state).unwrap();
}
