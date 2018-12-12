use crate::fields::{dirs4, Dir2D, Pos2D};
use crate::utils::umap::UMap;

use std::collections::HashMap;

pub trait AutomatonCell: Clone + Copy + PartialEq {
    fn update(&self, neighborhood: &Neighborhood<Self>) -> Self;

    fn new(pos: &Pos2D) -> Self;
}

pub trait Neighborhood<C: AutomatonCell> {
    fn find_cell(&self, pos: &Pos2D) -> &C;

    fn neumann(&self, pos: &Pos2D) -> HashMap<Dir2D, &C> {
        let mut map = HashMap::new();
        dirs4.iter().for_each(|&dir| {
            map.insert(dir, self.find_cell(&pos.move_by_one(dir)));
        });
        map
    }
}

#[derive(Default, Clone)]
pub struct Board<C: AutomatonCell> {
    pub dim: usize,
    pub map: UMap<C>,
}

fn get_id(dim: usize, pos: &Pos2D) -> usize {
    fn wrap(i: i64, dim: usize) -> usize {
        match i % dim as i64 {
            x if x >= 0 => x as usize,
            x => (x + dim as i64) as usize,
        }
    }

    wrap(pos.x, dim) * dim + wrap(pos.y, dim)
}

impl<C: AutomatonCell> Neighborhood<C> for Board<C> {
    fn find_cell(&self, pos: &Pos2D) -> &C {
        let id = get_id(self.dim, pos);
        self.map.get_ref(id).unwrap()
    }
}

impl<C: AutomatonCell> Board<C> {
    pub fn new(dim: usize) -> Self {
        let mut map = UMap::<C>::with_capacity(dim);
        Pos2D::from_dim(dim).iter().for_each(|pos| {
            let id = get_id(dim, pos);
            map.put(id, C::new(pos))
        });

        Board { dim, map }
    }

    pub fn update(&self) -> Self {
        let mut map = UMap::<C>::with_capacity(self.dim);
        Pos2D::from_dim(self.dim).iter().for_each(|pos| {
            let id = get_id(self.dim, pos);
            if let Some(ref cell) = self.map.get_ref(id) {
                map.put(id, cell.update(self))
            }
        });

        Board { dim: self.dim, map }
    }

    pub fn change_one(&self, position: &Pos2D, f: impl Fn(&C) -> C) -> Self {
        let mut map = UMap::<C>::with_capacity(self.dim);
        Pos2D::from_dim(self.dim).iter().for_each(|pos| {
            let id = get_id(self.dim, pos);
            if let Some(ref cell) = self.map.get_ref(id) {
                if pos == position {
                    map.put(id, f(cell));
                } else {
                    map.put(id, **cell)
                }
            }
        });

        Board { dim: self.dim, map }
    }
}

#[derive(Default, Clone)]
pub struct Automaton<C: AutomatonCell>(pub Board<C>);

impl<C: AutomatonCell> Automaton<C> {
    pub fn new(dim: usize) -> Automaton<C> {
        Automaton(Board::<C>::new(dim))
    }

    pub fn update(&mut self) {
        self.0 = self.0.update();
    }

    pub fn change(&mut self, f: impl Fn(&Board<C>) -> Board<C>) {
        self.0 = f(&self.0);
    }
}

impl<C: AutomatonCell> Iterator for Automaton<C> {
    type Item = Board<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.update();
        Some(self.0.clone())
    }
}
