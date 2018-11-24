use fields::{Dir2D, Pos2D, WhiteBlack};

use engine::rc_automaton::*;

use langtonsant::visualisation::Visualisation;

use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct RcLangtonsAnt {
    pub color: WhiteBlack,
    pub dir: Option<Dir2D>,
    pub pos: Pos2D,
    grid: Weak<RcGrid<RcLangtonsAnt>>,
}

impl fmt::Debug for RcLangtonsAnt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RcLangtonsAnt(color: {:?}, dir: {:?}, pos: {:?})",
            self.color, self.dir, self.pos
        )
    }
}

impl PartialEq for RcLangtonsAnt {
    fn eq(&self, other: &RcLangtonsAnt) -> bool {
        self.pos == other.pos
    }
}

impl RcLangtonsAnt {
    pub fn new_ant(pos: &Pos2D, grid: Weak<RcGrid<RcLangtonsAnt>>) -> Self {
        RcLangtonsAnt {
            color: WhiteBlack::White,
            dir: Some(Dir2D::Up),
            pos: *pos,
            grid,
        }
    }

    #[inline]
    fn grid(&self) -> Rc<RcGrid<RcLangtonsAnt>> {
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
                WhiteBlack::White => ant_dir.turn_right(),
                WhiteBlack::Black => ant_dir.turn_left(),
            })
        } else {
            None
        }
    }
}

impl RcAutomatonCell for RcLangtonsAnt {
    fn new(pos: Pos2D, grid: Weak<RcGrid<Self>>) -> Self {
        RcLangtonsAnt {
            color: WhiteBlack::White,
            dir: None,
            pos,
            grid,
        }
    }

    fn update(&self) -> Self {
        RcLangtonsAnt {
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

impl Visualisation for RcGrid<RcLangtonsAnt> {
    fn grid(&self) -> (Vec<char>, usize) {
        fn to_char(cell: &Option<Weak<RcLangtonsAnt>>) -> char {
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

impl fmt::Debug for RcNeighborhood<RcLangtonsAnt> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        let up = vec[0].upgrade().unwrap();
        let right = vec[1].upgrade().unwrap();
        let down = vec[2].upgrade().unwrap();
        let left = vec[3].upgrade().unwrap();

        write!(
            f,
            "RcNeighborhood<RcLangtonsAnt>(\nUP   : {:?},\nRIGHT: {:?},\nDOWN : {:?},\nLEFT : {:?})",
            up, right, down, left
        )
    }
}
