use crate::fields::{Dir2D, Pos2D, DIRS4};
use crate::utils::umap::UMap;

use crate::utils::umap::UMapIter;
use std::collections::HashMap;

pub trait AutomatonCell: Clone + Copy + PartialEq + Sized {
    fn update(&self, neighborhood: &Neighborhood<Self>) -> Self;
    fn position(&self) -> Pos2D;

    fn new(pos: &Pos2D) -> Self;
}

pub trait Neighborhood<C: AutomatonCell> {
    fn find_cell(&self, pos: &Pos2D) -> &C;

    fn neumann(&self, pos: &Pos2D) -> HashMap<Dir2D, &C> {
        let mut map = HashMap::new();
        DIRS4.iter().for_each(|&dir| {
            map.insert(dir, self.find_cell(&pos.move_by_one(dir)));
        });
        map
    }
}

#[derive(Default, Clone)]
pub struct Board<C: AutomatonCell> {
    dim: usize,
    map: UMap<C>,
}

impl<C: AutomatonCell> Board<C> {
    fn pos2id(dim: usize, pos: &Pos2D) -> usize {
        fn wrap(i: i64, dim: usize) -> usize {
            match i % dim as i64 {
                x if x >= 0 => x as usize,
                x => (x + dim as i64) as usize,
            }
        }

        wrap(pos.x, dim) * dim + wrap(pos.y, dim)
    }

    pub fn new(dim: usize) -> Self {
        let mut map = UMap::<C>::with_capacity(dim);
        Pos2D::from_dim(dim)
            .iter()
            .for_each(|pos| map.put(Board::<C>::pos2id(dim, pos), C::new(pos)));

        Board { dim, map }
    }

    pub fn update(&self) -> Self {
        let mut map = UMap::<C>::with_capacity(self.dim);
        Pos2D::from_dim(self.dim).iter().for_each(|pos| {
            let id = Board::<C>::pos2id(self.dim, pos);
            if let Some(ref cell) = self.map.get_ref(id) {
                map.put(id, cell.update(self))
            }
        });

        Board { dim: self.dim, map }
    }

    pub fn copy_and_update_one(&self, new_cell: &C) -> Self {
        self.copy_and_update(&[*new_cell])
    }

    pub fn copy_and_update(&self, cells: &[C]) -> Self {
        let mut map = UMap::<C>::with_capacity(self.dim);
        map.clone_from(&self.map);

        for c in cells {
            let id = Board::<C>::pos2id(self.dim, &c.position());
            map.put(id, *c);
        }

        Board { dim: self.dim, map }
    }

    pub fn copy_and_update_2(&self, cells: &UMap<C>) -> Self {
        let mut map = UMap::<C>::with_capacity(self.dim);
        map.clone_from(&self.map);
        cells.iter().for_each(|(id, cell)| map.put(id, *cell));
        Board { dim: self.dim, map }
    }
}

impl<C: AutomatonCell> Neighborhood<C> for Board<C> {
    fn find_cell(&self, pos: &Pos2D) -> &C {
        self.map.get_ref(Board::<C>::pos2id(self.dim, pos)).unwrap()
    }
}

#[derive(Default, Clone)]
pub struct Automaton<C: AutomatonCell> {
    board: Board<C>,
    changes: UMap<C>,
}

impl<C: AutomatonCell> Automaton<C> {
    pub fn new(dim: usize) -> Automaton<C> {
        Automaton {
            board: Board::<C>::new(dim),
            changes: UMap::<C>::with_capacity(dim),
        }
    }

    pub fn next(&mut self) {
        self.apply_changes();
        self.board = self.board.update();
    }

    pub fn transform(&mut self, f: impl Fn(&Board<C>) -> Board<C>) {
        self.board = f(&self.board);
    }

    pub fn add_change(&mut self, changed_cell: &C) {
        self.changes.put(
            Board::<C>::pos2id(self.dim(), &changed_cell.position()),
            *changed_cell,
        );
    }

    pub fn apply_changes(&mut self) {
        let cs = self.changes.clone();
        self.board = self.board.copy_and_update_2(&cs);
        self.changes = UMap::<C>::with_capacity(self.dim()); // TODO: implement `clear` for `UMap`
    }

    #[inline]
    pub fn dim(&self) -> usize {
        self.board.dim
    }

    #[inline]
    pub fn board_iter(&self) -> UMapIter<C> {
        self.board.map.iter()
    }
}

impl<C: AutomatonCell> Iterator for Automaton<C> {
    type Item = Board<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.board.update();
        Some(self.board.clone())
    }
}
