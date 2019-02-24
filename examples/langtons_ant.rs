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
use gailibrary::fields::{Pos2D, RGB};
use gailibrary::langtonsant::langtons_ant::LangtonsAnt;
use ggez::event::Keycode;
use ggez::event::Mod;

#[derive(Debug, Clone, Copy)]
struct CellRectangle {
    position: Pos2D,
    color: RGB,
}

struct MainState {
    cell_size: usize,

    iteration: usize,
    pause: bool,

    auto: Automaton<LangtonsAnt>,
}

impl MainState {
    pub fn new(_ctx: &mut Context, dim: usize) -> GameResult<MainState> {
        Ok(MainState {
            iteration: 0,
            cell_size: WINDOW_SIZE / dim,
            pause: false,
            auto: Automaton::<LangtonsAnt>::new(dim),
        })
    }

    pub fn add_ant(&mut self, ant_pos: Pos2D) {
        self.auto.add_change(&LangtonsAnt::new_ant(&ant_pos));
    }

    pub fn update(&mut self) {
        if self.iteration > 0 {
            self.auto.next();
        }
        self.iteration += 1;
    }

    pub fn draw_cells(&mut self, ctx: &mut Context) {
        for (color, group) in &self.auto2cells().into_iter().group_by(|c| c.color) {
            graphics::set_color(ctx, Color::from_rgb(color.r, color.g, color.b)).unwrap();
            self.draw(ctx, group.into_iter().map(|c| c.position).collect());
        }
    }

    pub fn pixels2pos(&self, x: usize, y: usize) -> Pos2D {
        Pos2D::new((x / self.cell_size) as i64, (y / self.cell_size) as i64)
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
                mesh.draw(ctx, Point2::new(0.0, 0.0), 0.0).unwrap();
            }
            Err(_) => {}
        };
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
        let res: Vec<CellRectangle> = self
            .auto
            .board_iter()
            .map(|(_, cell)| to_cell(cell))
            .filter(|cell| cell.color != RGB::WHITE)
            .collect();
        res
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.pause {
            self.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.pause {
            graphics::clear(ctx);
            self.draw_cells(ctx);
            graphics::present(ctx);
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: event::MouseButton,
        x: i32,
        y: i32,
    ) {
        let ant_pos = self.pixels2pos(x as usize, y as usize);
        println!("Button clicked at: {}", ant_pos);
        self.auto.add_change(&LangtonsAnt::new_ant(&ant_pos))
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if _keycode == Keycode::Space {
            self.pause = !self.pause;
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("langtons_ant", "makingthematrix", c).unwrap();
    graphics::set_background_color(ctx, graphics::WHITE);
    graphics::set_mode(
        ctx,
        WindowMode::default().dimensions(WINDOW_SIZE as u32, WINDOW_SIZE as u32),
    )
    .unwrap();
    graphics::set_screen_coordinates(
        ctx,
        Rect {
            x: 0.0,
            y: 0.0,
            w: WINDOW_SIZE as f32,
            h: WINDOW_SIZE as f32,
        },
    )
    .unwrap();

    let dim = 100;
    let state = &mut MainState::new(ctx, dim).unwrap();
    state.add_ant(Pos2D::new((dim as i64) / 2, (dim as i64) / 2));
    event::run(ctx, state).unwrap();
}
