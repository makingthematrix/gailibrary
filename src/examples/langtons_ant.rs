use crate::engine::automaton::*;
use crate::fields::{Dir2D, Pos2D};

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct LangtonsAnt {
    pub color: bool,
    pub dir: Option<Dir2D>,
    pub pos: Pos2D,
}

impl LangtonsAnt {
    pub fn new_ant(pos: &Pos2D) -> Self {
        LangtonsAnt {
            color: true,
            dir: Some(Dir2D::Up),
            pos: *pos,
        }
    }

    fn update_color(&self) -> bool {
        if self.dir.is_some() {
            !self.color
        } else {
            self.color
        }
    }

    fn update_dir(&self, neighborhood: &dyn Neighborhood<Self>) -> Option<Dir2D> {
        if let Some((ref ant_dir, ..)) = neighborhood
            .neumann(&self.pos)
            .iter()
            .find(|&(d, &c)| c.dir.map_or(false, |c_dir| c_dir == d.turn_around()))
        {
            Some(if self.color {
                ant_dir.turn_right()
            } else {
                ant_dir.turn_left()
            })
        } else {
            None
        }
    }
}

impl AutomatonCell for LangtonsAnt {
    fn update(&self, neighborhood: &dyn Neighborhood<Self>) -> Self {
        LangtonsAnt {
            color: self.update_color(),
            dir: self.update_dir(neighborhood),
            pos: self.pos,
        }
    }

    fn position(&self) -> Pos2D {
        self.pos
    }

    fn new(pos: &Pos2D) -> Self {
        LangtonsAnt {
            color: false,
            dir: None,
            pos: *pos,
        }
    }
}

impl fmt::Debug for LangtonsAnt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LangtonsAnt({:?} -> color: {:?}, dir: {:?})",
            self.pos, self.color, self.dir
        )
    }
}
