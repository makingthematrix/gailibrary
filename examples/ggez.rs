//! The simplest possible example that does something.
extern crate ggez;

use ggez::conf;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, *};
use ggez::{Context, GameResult};

use itertools::Itertools;

const WINDOW_SIZE: usize = 800;

use gailibrary::engine::automaton::Automaton;
use gailibrary::fields::{Dir2D, Pos2D, RGB};
use gailibrary::langtonsant::langtons_ant::LangtonsAnt;

#[derive(Debug, Clone, Copy)]
struct CellRectangle {
    position: Pos2D,
    color: RGB,
}

struct MainState {
    iteration: usize,
    cell_size: usize,
    auto: Automaton<LangtonsAnt>,
}

impl MainState {
    pub fn new(_ctx: &mut Context, dim: usize) -> GameResult<MainState> {
        Ok(MainState {
            iteration: 0,
            cell_size: WINDOW_SIZE / dim,
            auto: Automaton::<LangtonsAnt>::new(dim),
        })
    }

    pub fn add_ant(&mut self, ant_pos: Pos2D) {
        self.auto
            .change(|board| board.change_one(&ant_pos, |_| LangtonsAnt::new_ant(&ant_pos)));
    }

    pub fn update(&mut self) {
        if self.iteration > 0 {
            self.auto.next();
        }
        self.iteration += 1;
    }

    fn rgb2color(rgb: RGB) -> Color {
        Color::from_rgb(rgb.r, rgb.g, rgb.b)
    }

    fn draw(&mut self, ctx: &mut Context, positions: Vec<Pos2D>) {
        let mut builder = MeshBuilder::new();
        positions.iter().for_each(|p| {
            let x1 = (p.x as usize * self.cell_size) as f32;
            let x2 = x1 + (self.cell_size as f32);
            let y1 = (p.y as usize * self.cell_size) as f32;
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
            }
            Err(_) => {}
        };
    }

    fn draw_cells(&mut self, ctx: &mut Context) {
        for (color, group) in &self.auto2cells().into_iter().group_by(|c| c.color) {
            if color != RGB::WHITE {
                graphics::set_color(ctx, MainState::rgb2color(color));
                self.draw(ctx, group.into_iter().map(|c| c.position).collect());
            }
        }
    }

    fn auto2cells(&self) -> Vec<CellRectangle> {
        fn to_cell(cell: &LangtonsAnt) -> CellRectangle {
            let position = cell.pos;
            let color = match cell.dir {
                None => {
                    if cell.color {
                        RGB::BLACK
                    } else {
                        RGB::WHITE
                    }
                }
                Some(_) => RGB::RED,
            };
            CellRectangle { position, color }
        }
        let res: Vec<CellRectangle> = self.auto.0.map.iter().map(|(_, cell)| to_cell(cell)).collect();
        res
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update();
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
    graphics::set_mode(
        ctx,
        WindowMode::default().dimensions(WINDOW_SIZE as u32, WINDOW_SIZE as u32),
    );
    graphics::set_screen_coordinates(
        ctx,
        Rect {
            x: 0.0,
            y: 0.0,
            w: WINDOW_SIZE as f32,
            h: WINDOW_SIZE as f32,
        },
    );

    let dim = 100;
    let state = &mut MainState::new(ctx, dim).unwrap();
    state.add_ant(Pos2D::new((dim as i64) / 2, (dim as i64) / 2));
    event::run(ctx, state).unwrap();
}
