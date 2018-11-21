use engine::grid::Grid;
use std::rc::{Rc, Weak};

use enums::pos_2d::Pos2D;
use utils::umap::UMap;

pub trait ArenaCell: Clone + PartialEq {
    fn new(pos: Pos2D, grid: Weak<Grid<Self>>) -> Self;
    fn update(&self) -> Self;
    fn pos(&self) -> &Pos2D;
}

pub struct Arena<T: ArenaCell>(UMap<Rc<T>>);

impl<T: ArenaCell> Arena<T> {
    pub fn new(capacity: usize) -> Self {
        Arena(UMap::<Rc<T>>::with_capacity(capacity))
    }

    pub fn init(&mut self, grid: &Rc<Grid<T>>) {
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
        Arena(umap)
    }

    pub fn update(&self) -> Self {
        let updated: Vec<(usize, Rc<T>)> = self
            .0
            .iter()
            .map(|(key, value)| (key, Rc::new(value.update())))
            .collect();
        let umap: UMap<Rc<T>> = updated.into();
        Arena(umap)
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
