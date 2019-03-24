use crate::engine::automaton::*;
use crate::fields::Pos2D;

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct GameOfLife {
    pub life: bool,
    pub pos: Pos2D,
}

impl GameOfLife {
    pub fn new_life(pos: &Pos2D) -> Self {
        GameOfLife {
            life: true,
            pos: *pos,
        }
    }

    fn update_life(&self, neighborhood: &Neighborhood<Self>) -> bool {
        match neighborhood
            .moore(&self.pos)
            .iter()
            .filter(|&(_, &c)| c.life)
            .count()
        {
            3 if !self.life => true,
            n if self.life && (n < 2 || n > 3) => false,
            _ => self.life,
        }
    }
}

impl AutomatonCell for GameOfLife {
    fn update(&self, neighborhood: &Neighborhood<Self>) -> Self {
        GameOfLife {
            life: self.update_life(neighborhood),
            pos: self.pos,
        }
    }

    fn position(&self) -> Pos2D {
        self.pos
    }

    fn new(pos: &Pos2D) -> Self {
        GameOfLife {
            life: false,
            pos: *pos,
        }
    }
}

impl fmt::Debug for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameOfLife({:?} -> {:?})", self.pos, self.life)
    }
}
