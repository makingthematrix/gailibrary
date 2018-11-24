use engine::automaton::*;

use fields::{Dir2D, Pos2D};

use langtonsant::visualisation::Visualisation;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LAnt {
    pub color: bool,
    pub dir: Option<Dir2D>,
    pub pos: Pos2D,
}

impl LAnt {
    pub fn new_ant(pos: &Pos2D) -> Self {
        LAnt {
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

    fn update_dir(&self, neighborhood: &Neighborhood<Self>) -> Option<Dir2D> {
        if let Some((ref ant_dir, ..)) = neighborhood
            .neumann(&self.pos)
            .iter()
            .find(|&(d, &c)| c.dir.map_or(false, |c_dir| c_dir == d.turn_around()))
        {
            Some(if self.color {
                ant_dir.turn_left()
            } else {
                ant_dir.turn_right()
            })
        } else {
            None
        }
    }
}

impl AutomatonCell for LAnt {
    fn update(&self, neighborhood: &Neighborhood<Self>) -> Self {
        LAnt {
            color: self.update_color(),
            dir: self.update_dir(neighborhood),
            pos: self.pos,
        }
    }

    fn new(pos: &Pos2D) -> Self {
        LAnt {
            color: false,
            dir: None,
            pos: *pos,
        }
    }
}

impl Visualisation for Board<LAnt> {
    fn grid(&self) -> (Vec<char>, usize) {
        fn to_char(cell: &LAnt) -> char {
            match cell.dir {
                None => {
                    if cell.color {
                        'X'
                    } else {
                        '_'
                    }
                }
                Some(Dir2D::Up) => '<',
                Some(Dir2D::Right) => 'v',
                Some(Dir2D::Down) => '>',
                Some(Dir2D::Left) => '^',
            }
        }

        let res: Vec<char> = self.map.iter().map(|(_, cell)| to_char(cell)).collect();
        (res, self.dim)
    }
}
