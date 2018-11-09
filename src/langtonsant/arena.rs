use langtonsant::grid::Grid;
use langtonsant::langtons_ant::LangtonsAnt;
use std::rc::{Rc, Weak};

use utils::umap::UMap;

pub struct Arena<T>(UMap<Rc<T>>);

impl Arena<LangtonsAnt> {
    pub fn new(capacity: usize) -> Self {
        Arena(UMap::<Rc<LangtonsAnt>>::with_capacity(capacity))
    }

    pub fn init(&mut self, grid: &Rc<Grid<LangtonsAnt>>) {
        let dim = grid.dim();
        for i in 0..dim {
            for j in 0..dim {
                self.0.put(
                    i * dim + j,
                    Rc::new(LangtonsAnt::new(i, j, Rc::downgrade(grid))),
                );
            }
        }
    }

    pub fn insert(&self, id: usize, cell: &Rc<LangtonsAnt>) -> Self {
        let updated: Vec<(usize, Rc<LangtonsAnt>)> = self
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
        let umap: UMap<Rc<LangtonsAnt>> = updated.into();
        Arena(umap)
    }

    pub fn update(&self) -> Self {
        let updated: Vec<(usize, Rc<LangtonsAnt>)> = self
            .0
            .iter()
            .map(|(key, value)| (key, Rc::new(value.update())))
            .collect();
        let umap: UMap<Rc<LangtonsAnt>> = updated.into();
        Arena(umap)
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        Self: Sized,
        F: FnMut(&Rc<LangtonsAnt>),
    {
        self.0.iter().for_each(|(.., value)| f(&value));
    }

    pub fn get(&self, id: usize) -> Option<Weak<LangtonsAnt>> {
        self.0.get(id).map(|c| Rc::downgrade(&c))
    }
}
