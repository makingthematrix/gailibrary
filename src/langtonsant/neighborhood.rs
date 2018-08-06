use std::fmt;
use std::rc::Weak;

use enums::dir_2d::Dir2D;
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

    pub fn find<F>(&self, pred: F) -> Option<(Weak<LangtonsAnt>, Dir2D)>
    where
        F: Fn(&LangtonsAnt, Dir2D) -> bool,
    {
        let dir = Dir2D::iter().find(|&dir| {
            let vec = &self.0;
            let index = dir as usize;

            let link = &vec[index];
            if let Some(cell) = link.upgrade() {
                pred(&cell, dir)
            } else {
                false
            }
        });

        dir.map(|d| (self.0[d as usize].clone(), d))
    }
}

impl fmt::Debug for Neighborhood<LangtonsAnt> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        let u = vec[0].upgrade().unwrap();
        let r = vec[1].upgrade().unwrap();
        let d = vec[2].upgrade().unwrap();
        let l = vec[3].upgrade().unwrap();

        write!(
            f,
            "Neighborhood<LangtonsAnt>(\nUP   : {:?},\nRIGHT: {:?},\nDOWN : {:?},\nLEFT : {:?})",
            u, r, d, l
        )
    }
}
