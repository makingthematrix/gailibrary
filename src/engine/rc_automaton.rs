use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use utils::umap::UMap;

use fields::{dirs4, Dir2D, Pos2D};

pub trait RcAutomatonCell: Clone + PartialEq {
    fn new(pos: Pos2D, grid: Weak<RcGrid<Self>>) -> Self;
    fn update(&self) -> Self;
    fn pos(&self) -> &Pos2D;
}

pub struct RcArena<T: RcAutomatonCell>(UMap<Rc<T>>);

impl<T: RcAutomatonCell> RcArena<T> {
    pub fn new(capacity: usize) -> Self {
        RcArena(UMap::<Rc<T>>::with_capacity(capacity))
    }

    pub fn init(&mut self, grid: &Rc<RcGrid<T>>) {
        let dim = grid.dim();

        Pos2D::from_dim(dim).iter().for_each(|pos| {
            self.0
                .put(grid.id(pos), Rc::new(T::new(*pos, Rc::downgrade(grid))));
        });
    }

    pub fn insert(&self, id: usize, cell: &Rc<T>) -> Self {
        let updated: Vec<(usize, Rc<T>)> = self
            .0
            .iter()
            .map(|(key, value)| {
                if key != id {
                    (key, value.clone())
                } else {
                    (id, cell.clone())
                }
            })
            .collect();
        let umap: UMap<Rc<T>> = updated.into();
        RcArena(umap)
    }

    pub fn update(&self) -> Self {
        let updated: Vec<(usize, Rc<T>)> = self
            .0
            .iter()
            .map(|(key, value)| (key, Rc::new(value.update())))
            .collect();
        let umap: UMap<Rc<T>> = updated.into();
        RcArena(umap)
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        Self: Sized,
        F: FnMut(&Rc<T>),
    {
        self.0.iter().for_each(|(.., value)| f(&value));
    }

    pub fn get(&self, id: usize) -> Option<Weak<T>> {
        self.0.get(id).map(|c| Rc::downgrade(&c))
    }
}

pub struct RcGrid<T: RcAutomatonCell> {
    vec: RefCell<Vec<Option<Weak<T>>>>,
    dim: usize,
}

impl<T: RcAutomatonCell> RcGrid<T> {
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
        RcGrid {
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

    pub fn update(&self, arena: &RcArena<T>) {
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

    pub fn all_near(&self, cell: &T) -> RcNeighborhood<T> {
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

        RcNeighborhood::new(u, r, d, l)
    }
}

pub struct RcNeighborhood<T>(pub Vec<Weak<T>>);

impl<T> RcNeighborhood<T> {
    pub fn new(up: Weak<T>, right: Weak<T>, down: Weak<T>, left: Weak<T>) -> Self {
        RcNeighborhood(vec![up, right, down, left])
    }

    fn get_link(&self, dir: &Dir2D) -> &Weak<T> {
        match *dir {
            Dir2D::Up => &self.0[0],
            Dir2D::Right => &self.0[1],
            Dir2D::Down => &self.0[2],
            Dir2D::Left => &self.0[3],
        }
    }

    pub fn find<F>(&self, pred: F) -> Option<(Weak<T>, Dir2D)>
    where
        F: Fn(&T, Dir2D) -> bool,
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

pub struct RcAutomaton<C: RcAutomatonCell> {
    arena: RcArena<C>,
    pub grid: Rc<RcGrid<C>>,
}

impl<C: RcAutomatonCell> RcAutomaton<C> {
    pub fn new(dim: usize) -> Self {
        let mut arena = RcArena::<C>::new(dim * dim);
        let grid = Rc::new(RcGrid::<C>::new(dim));

        arena.init(&grid);
        grid.update(&arena);

        RcAutomaton { arena, grid }
    }

    pub fn get_grid(&self) -> Weak<RcGrid<C>> {
        Rc::downgrade(&self.grid)
    }

    pub fn update(&mut self) {
        let arena = self.arena.update();
        self.grid.update(&arena);
        self.arena = arena;
    }

    pub fn insert(&mut self, pos: &Pos2D, cell: &Rc<C>) {
        let id = self.grid.insert(pos, Rc::downgrade(cell));
        self.arena = self.arena.insert(id, cell);
    }
}
