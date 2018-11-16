use engine::arena::{Arena, ArenaCell};
use engine::neighborhood::Neighborhood;

use enums::dir_2d::Dir2D;
use enums::pos_2d::Pos2D;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub struct Grid<T: ArenaCell> {
    vec: RefCell<Vec<Option<Weak<T>>>>,
    dim: usize,
}

impl<T: ArenaCell> Grid<T> {
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

    pub fn vec(&self) -> Ref<Vec<Option<Weak<T>>>> {
        self.vec.borrow()
    }

    pub fn update(&self, arena: &Arena<T>) {
        let mut ref_mut = self.vec.borrow_mut();
        arena.for_each(|cell| {
            ref_mut[self.id(cell.pos())] = Some(Rc::downgrade(cell));
        });
    }

    pub fn insert(&self, pos: &Pos2D, cell: Weak<T>) -> usize {
        let mut ref_mut = self.vec.borrow_mut();
        let id = self.id(pos);
        ref_mut[id] = Some(cell);
        id
    }

    pub fn exists<F>(&self, index: usize, pred: F) -> bool
    where
        F: Fn(&T) -> bool,
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
    pub fn get(&self, pos: &Pos2D) -> Option<Weak<T>> {
        let rf = self.vec.borrow();
        rf[self.id(pos)].clone()
    }

    pub fn all_near(&self, cell: &T) -> Neighborhood<T> {
        let u = self
            .get(&cell.pos().move_by_one(Dir2D::Up))
            .unwrap()
            .clone();
        let r = self
            .get(&cell.pos().move_by_one(Dir2D::Right))
            .unwrap()
            .clone();
        let d = self
            .get(&cell.pos().move_by_one(Dir2D::Down))
            .unwrap()
            .clone();
        let l = self
            .get(&cell.pos().move_by_one(Dir2D::Left))
            .unwrap()
            .clone();

        Neighborhood::new(u, r, d, l)
    }
}
