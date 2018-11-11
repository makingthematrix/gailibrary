use std::fmt;
use std::rc::Weak;

use enums::dir_2d::{dirs4, Dir2D};
use langtonsant::langtons_ant::LangtonsAnt;

pub struct Neighborhood<T>(Vec<Weak<T>>);

impl Neighborhood<LangtonsAnt> {
    pub fn new(
        up: Weak<LangtonsAnt>,
        right: Weak<LangtonsAnt>,
        down: Weak<LangtonsAnt>,
        left: Weak<LangtonsAnt>,
    ) -> Self {
        Neighborhood(vec![up, right, down, left])
    }

    fn get_link(&self, dir: &Dir2D) -> &Weak<LangtonsAnt> {
        match *dir {
            Dir2D::Up => &self.0[0],
            Dir2D::Right => &self.0[1],
            Dir2D::Down => &self.0[2],
            Dir2D::Left => &self.0[3],
        }
    }

    pub fn find<F>(&self, pred: F) -> Option<(Weak<LangtonsAnt>, Dir2D)>
    where
        F: Fn(&LangtonsAnt, Dir2D) -> bool,
    {
        let dir = dirs4.iter().find(|&dir| {
            if let Some(cell) = self.get_link(dir).upgrade() {
                pred(&cell, *dir)
            } else {
                false
            }
        });

        dir.map(|&d| (self.0[d as usize].clone(), d))
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
