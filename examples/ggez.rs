//! The simplest possible example that does something.

extern crate ggez;

use ggez::conf;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, *};
use ggez::{Context, GameResult};

use itertools::Itertools;

const WINDOW_SIZE: u32 = 800;

use gailibrary::fields::{Pos2D, RGB};


#[derive(Debug, Clone, Copy)]
struct CellRectangle {
    position: Pos2D,
    color: RGB
}

struct MainState {
    dim: u32,
    cell_size: u32,

    cells: Vec<CellRectangle>
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { dim: 100, cell_size: WINDOW_SIZE / 100, cells: Vec::default() };
        Ok(s)
    }

    fn rgb2color(rgb: RGB) -> Color {
        Color::from_rgb(rgb.r, rgb.g, rgb.b)
    }

    fn draw(&mut self, ctx: &mut Context, positions: Vec<Pos2D>) {
        let mut builder = MeshBuilder::new();
        positions.iter().for_each(|p| {
            let x1 = (p.x as u32 * self.cell_size) as f32;
            let x2 = x1 + (self.cell_size as f32);
            let y1 = (p.y  as u32 * self.cell_size) as f32;
            let y2 = y1 + (self.cell_size as f32);
            let pts = [
                Point2::new(x1, y1),
                Point2::new(x2, y1),
                Point2::new(x2, y2),
                Point2::new(x1, y2),
            ];
            builder.polygon(DrawMode::Fill, &pts);
        });
        match builder.build(ctx) {
            Ok(mesh) => {
                mesh.draw(ctx, Point2::new(0.0, 0.0), 0.0);
            },
            Err(_) => {}
        };
    }

    fn draw_cells(&mut self, ctx: &mut Context) {
        let cs = self.cells.to_vec();
        for (color, group) in &cs.into_iter().group_by(|c| c.color) {
            graphics::set_color(ctx, MainState::rgb2color(color));
            self.draw(ctx, group.into_iter().map(|c| c.position).collect());
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.cells = vec!(
            CellRectangle { position: Pos2D::new(1, 1), color: RGB::RED },
            CellRectangle { position: Pos2D::new(3, 4), color: RGB::RED },
            CellRectangle { position: Pos2D::new(7, 4), color: RGB::GREEN },
            CellRectangle { position: Pos2D::new(13, 4), color: RGB::GREEN }
        );
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.draw_cells(ctx);

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    graphics::set_background_color(ctx, graphics::WHITE);
    graphics::set_mode(ctx, WindowMode::default().dimensions(WINDOW_SIZE, WINDOW_SIZE));
    graphics::set_screen_coordinates(ctx, Rect { x: 0.0, y: 0.0, w: WINDOW_SIZE as f32, h: WINDOW_SIZE as f32 });
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}