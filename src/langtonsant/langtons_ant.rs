use enums::dir_2d::*;
use enums::pos_2d::*;
use enums::white_black::*;

use engine::grid::Grid;

use engine::arena::ArenaCell;
use engine::neighborhood::Neighborhood;
use langtonsant::visualisation::Visualisation;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct LangtonsAnt {
    pub color: WhiteBlack,
    pub dir: Option<Dir2D>,
    pub pos: Pos2D,
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
        self.pos == other.pos
    }
}

impl LangtonsAnt {
    pub fn new_ant(pos: &Pos2D, grid: Weak<Grid<LangtonsAnt>>) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: Some(Dir2D::Up),
            pos: *pos,
            grid,
        }
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
}

impl ArenaCell for LangtonsAnt {
    fn new(pos: Pos2D, grid: Weak<Grid<LangtonsAnt>>) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: None,
            pos,
            grid,
        }
    }

    fn update(&self) -> Self {
        LangtonsAnt {
            color: self.update_color(),
            dir: self.update_dir(),
            pos: self.pos,
            grid: self.grid.clone(),
        }
    }

    fn pos(&self) -> &Pos2D {
        &self.pos
    }
}

impl Visualisation for Grid<LangtonsAnt> {
    fn grid(&self) -> (Vec<char>, usize) {
        fn to_char(cell: &Option<Weak<LangtonsAnt>>) -> char {
            if let Some(ref rf) = cell {
                if let Some(ant) = rf.upgrade() {
                    match ant.dir {
                        None => {
                            if ant.color == WhiteBlack::White {
                                '_'
                            } else {
                                'X'
                            }
                        }
                        Some(Dir2D::Up) => '<',
                        Some(Dir2D::Right) => 'v',
                        Some(Dir2D::Down) => '>',
                        Some(Dir2D::Left) => '^',
                    }
                } else {
                    '_'
                }
            } else {
                '_'
            }
        }

        let rf = self.vec();
        let res: Vec<char> = rf.iter().map(|c| to_char(c)).collect();
        (res, self.dim())
    }
}

impl fmt::Debug for Neighborhood<LangtonsAnt> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        let up = vec[0].upgrade().unwrap();
        let right = vec[1].upgrade().unwrap();
        let down = vec[2].upgrade().unwrap();
        let left = vec[3].upgrade().unwrap();

        write!(
            f,
            "Neighborhood<LangtonsAnt>(\nUP   : {:?},\nRIGHT: {:?},\nDOWN : {:?},\nLEFT : {:?})",
            up, right, down, left
        )
    }
}
