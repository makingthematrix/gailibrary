use langtonsant::arena::Arena;
use langtonsant::langtons_ant::LangtonsAnt;
use langtonsant::neighborhood::Neighborhood;
use langtonsant::visualisation::Visualisation;

use enums::dir_2d::Dir2D;
use enums::white_black::WhiteBlack;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub struct Grid<T> {
    vec: RefCell<Vec<Option<Weak<T>>>>,
    dim: usize,
}

impl Grid<LangtonsAnt> {
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
            ref_mut[cell.x() * self.dim + cell.y()] = Some(Rc::downgrade(cell));
        });
    }

    pub fn insert(&self, x: usize, y: usize, cell: Weak<LangtonsAnt>) {
        let mut ref_mut = self.vec.borrow_mut();
        ref_mut[x * self.dim + y] = Some(cell);
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
    pub fn get(&self, x: usize, y: usize) -> Option<Weak<LangtonsAnt>> {
        let rf = self.vec.borrow();
        rf[x * self.dim + y].clone()
    }

    #[inline]
    fn get_with_offset(&self, x: usize, offx: i64, y: usize, offy: i64) -> Weak<LangtonsAnt> {
        let xi: usize = if x == 0 && offx == -1 {
            self.dim - 1
        } else if x == self.dim - 1 && offx == 1 {
            0
        } else {
            (x as i64 + offx) as usize
        };
        let yi: usize = if y == 0 && offy == -1 {
            self.dim - 1
        } else if y == self.dim - 1 && offy == 1 {
            0
        } else {
            (y as i64 + offy) as usize
        };
        let rf = self.vec.borrow();
        rf[xi * self.dim + yi].clone().unwrap()
    }

    pub fn all_near(&self, cell: &LangtonsAnt) -> Neighborhood<LangtonsAnt> {
        let u = self.get_with_offset(cell.x(), 0, cell.y(), -1);
        let r = self.get_with_offset(cell.x(), 1, cell.y(), 0);
        let d = self.get_with_offset(cell.x(), 0, cell.y(), 1);
        let l = self.get_with_offset(cell.x(), -1, cell.y(), 0);

        Neighborhood::new(u, r, d, l)
    }
}

impl Visualisation for Grid<LangtonsAnt> {
    fn grid(&self) -> (Vec<char>, usize) {
        fn to_char(cell: &Option<Weak<LangtonsAnt>>) -> char {
            if let Some(ref rf) = cell {
                if let Some(ant) = rf.upgrade() {
                    match ant.dir() {
                        Dir2D::None => if ant.color() == WhiteBlack::White {
                            '_'
                        } else {
                            'X'
                        },
                        Dir2D::Up => '<',
                        Dir2D::Right => 'v',
                        Dir2D::Down => '>',
                        Dir2D::Left => '^',
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
