use enums::dir_2d::*;
use enums::white_black::*;

use langtonsant::grid::Grid;

use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct LangtonsAnt {
    color: WhiteBlack,
    dir: Option<Dir2D>,
    pos: (usize, usize),
    grid: Weak<Grid<LangtonsAnt>>,
}

impl fmt::Debug for LangtonsAnt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LangtonsAnt(color: {:?}, dir: {:?}, pos: {:?})",
            self.color, self.dir, self.pos
        )
    }
}

impl PartialEq for LangtonsAnt {
    fn eq(&self, other: &LangtonsAnt) -> bool {
        self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1
    }
}

impl LangtonsAnt {
    pub fn new(x: usize, y: usize, grid: Weak<Grid<LangtonsAnt>>) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: None,
            pos: (x, y),
            grid,
        }
    }

    pub fn new_ant(x: usize, y: usize, grid: Weak<Grid<LangtonsAnt>>) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: Some(Dir2D::Up),
            pos: (x, y),
            grid,
        }
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.pos.0
    }

    #[inline]
    pub fn y(&self) -> usize {
        self.pos.1
    }

    #[inline]
    pub fn dir(&self) -> Option<Dir2D> {
        self.dir
    }

    #[inline]
    pub fn color(&self) -> WhiteBlack {
        self.color
    }

    #[inline]
    fn grid(&self) -> Rc<Grid<LangtonsAnt>> {
        self.grid.upgrade().unwrap()
    }

    fn update_color(&self) -> WhiteBlack {
        self.dir.map_or(self.color, |_| self.color.toggle())
    }

    pub fn update_dir(&self) -> Option<Dir2D> {
        if let Some((.., ant_dir)) = self
            .grid()
            .all_near(self)
            .find(|c, d| c.dir.map_or(false, |c_dir| c_dir == d.turn_around()))
        {
            Some(match self.color {
                WhiteBlack::White => ant_dir.turn_left(),
                WhiteBlack::Black => ant_dir.turn_right(),
            })
        } else {
            None
        }
    }

    pub fn update(&self) -> Self {
        LangtonsAnt {
            color: self.update_color(),
            dir: self.update_dir(),
            pos: (self.pos.0, self.pos.1),
            grid: self.grid.clone(),
        }
    }
}
