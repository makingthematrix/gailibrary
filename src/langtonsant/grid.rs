use langtonsant::arena::Arena;
use langtonsant::langtons_ant::LangtonsAnt;
use langtonsant::neighborhood::Neighborhood;
use langtonsant::visualisation::Visualisation;

use enums::dir_2d::Dir2D;
use enums::pos_2d::Pos2D;
use enums::white_black::WhiteBlack;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub struct Grid<T> {
    vec: RefCell<Vec<Option<Weak<T>>>>,
    dim: usize,
}

impl Grid<LangtonsAnt> {
    pub fn id(&self, pos: &Pos2D) -> usize {
        fn wrap(i: i64, dim: usize) -> usize {
            match i % dim as i64 {
                x if x >= 0 => x as usize,
                x => (x + dim as i64) as usize,
            }
        }

        wrap(pos.x, self.dim) * self.dim + wrap(pos.y, self.dim)
    }

    pub fn new(dim: usize) -> Self {
        Grid {
            vec: RefCell::new(vec![None; dim * dim]),
            dim,
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn vec(&self) -> Ref<Vec<Option<Weak<LangtonsAnt>>>> {
        self.vec.borrow()
    }

    pub fn update(&self, arena: &Arena<LangtonsAnt>) {
        let mut ref_mut = self.vec.borrow_mut();
        arena.for_each(|cell| {
            ref_mut[self.id(&cell.pos)] = Some(Rc::downgrade(cell));
        });
    }

    pub fn insert(&self, pos: &Pos2D, cell: Weak<LangtonsAnt>) -> usize {
        let mut ref_mut = self.vec.borrow_mut();
        let id = self.id(pos);
        ref_mut[id] = Some(cell);
        id
    }

    pub fn exists<F>(&self, index: usize, pred: F) -> bool
    where
        F: Fn(&LangtonsAnt) -> bool,
    {
        let vec = self.vec.borrow();
        if index >= vec.len() {
            return false;
        }

        if let Some(ref link) = vec[index] {
            if let Some(cell) = link.upgrade() {
                pred(&cell)
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline]
    pub fn get(&self, pos: &Pos2D) -> Option<Weak<LangtonsAnt>> {
        let rf = self.vec.borrow();
        rf[self.id(pos)].clone()
    }

    pub fn all_near(&self, cell: &LangtonsAnt) -> Neighborhood<LangtonsAnt> {
        let u = self.get(&cell.pos.move_by_one(Dir2D::Up)).unwrap().clone();
        let r = self
            .get(&cell.pos.move_by_one(Dir2D::Right))
            .unwrap()
            .clone();
        let d = self
            .get(&cell.pos.move_by_one(Dir2D::Down))
            .unwrap()
            .clone();
        let l = self
            .get(&cell.pos.move_by_one(Dir2D::Left))
            .unwrap()
            .clone();

        Neighborhood::new(u, r, d, l)
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
