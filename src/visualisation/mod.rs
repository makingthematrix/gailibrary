extern crate ggez;

use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, *};
use ggez::{Context, GameResult};

use ggez::event::Keycode;
use ggez::event::Mod;

use itertools::Itertools;

use crate::engine::automaton::Automaton;
use crate::engine::automaton::AutomatonCell;
use crate::examples::game_of_life::GameOfLife;
use crate::examples::langtons_ant::LangtonsAnt;
use crate::fields::Pos2D;
use crate::fields::RGB;

#[derive(Debug, Clone, Copy)]
pub struct CellRectangle {
    position: Pos2D,
    color: RGB,
}

pub trait CellVisualisation: AutomatonCell {
    fn new_cell(pos: &Pos2D) -> Self;
    fn cell_to_rectangle(&self) -> CellRectangle;
}

pub struct MainState<C: CellVisualisation> {
    cell_size: usize,

    iteration: usize,
    pause: bool,

    auto: Automaton<C>,
}

impl<C: CellVisualisation> MainState<C> {
    pub fn new(window_size: usize, dim: usize) -> GameResult<MainState<C>> {
        Ok(MainState {
            iteration: 0,
            cell_size: window_size / dim,
            pause: false,
            auto: Automaton::<C>::new(dim),
        })
    }

    pub fn add(&mut self, pos: &Pos2D) {
        self.auto.add_change(&C::new_cell(pos));
    }

    fn update(&mut self) {
        if self.iteration > 0 {
            self.auto.next();
        }
        self.iteration += 1;
    }

    fn auto2cells(&self) -> Vec<CellRectangle> {
        let res: Vec<CellRectangle> = self
            .auto
            .board_iter()
            .map(|(_, cell)| cell.cell_to_rectangle())
            .filter(|cell| cell.color != RGB::WHITE)
            .collect();
        res
    }

    fn pixels2pos(&self, x: usize, y: usize) -> Pos2D {
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

        if let Ok(mesh) = builder.build(ctx) {
            mesh.draw(ctx, Point2::new(0.0, 0.0), 0.0).unwrap();
        };
    }

    fn draw_cells(&mut self, ctx: &mut Context) {
        for (color, group) in &self.auto2cells().into_iter().group_by(|c| c.color) {
            graphics::set_color(ctx, Color::from_rgb(color.r, color.g, color.b)).unwrap();
            self.draw(ctx, group.map(|c| c.position).collect());
        }
    }
}

impl<C: CellVisualisation> event::EventHandler for MainState<C> {
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
        ctx: &mut Context,
        _btn: event::MouseButton,
        x: i32,
        y: i32,
    ) {
        let pos = self.pixels2pos(x as usize, y as usize);
        println!("Button clicked at: {}", pos);
        self.pause = true;
        self.auto.add_change(&C::new_cell(&pos));
        self.auto.apply_changes();
        self.draw_cells(ctx);

        graphics::present(ctx);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if _keycode == Keycode::Space {
            self.pause = !self.pause;
        }
    }
}

pub fn setup(ctx: &mut Context, window_size: usize) {
    graphics::set_background_color(ctx, graphics::WHITE);
    graphics::set_mode(
        ctx,
        WindowMode::default().dimensions(window_size as u32, window_size as u32),
    )
    .unwrap();
    graphics::set_screen_coordinates(
        ctx,
        Rect {
            x: 0.0,
            y: 0.0,
            w: window_size as f32,
            h: window_size as f32,
        },
    )
    .unwrap();
}

impl CellVisualisation for LangtonsAnt {
    fn new_cell(pos: &Pos2D) -> Self {
        LangtonsAnt::new_ant(pos)
    }

    fn cell_to_rectangle(&self) -> CellRectangle {
        let position = self.pos;
        let color = match self.dir {
            None => {
                if self.color {
                    RGB::BLACK
                } else {
                    RGB::WHITE
                }
            }
            Some(_) => RGB::RED,
        };
        CellRectangle { position, color }
    }
}

impl CellVisualisation for GameOfLife {
    fn new_cell(pos: &Pos2D) -> Self {
        GameOfLife::new_life(pos)
    }

    fn cell_to_rectangle(&self) -> CellRectangle {
        let position = self.pos;
        let color = if self.life { RGB::BLACK } else { RGB::WHITE };
        CellRectangle { position, color }
    }
}
