use std::rc::Weak;

use enums::dir_2d::{dirs4, Dir2D};

pub struct Neighborhood<T>(pub Vec<Weak<T>>);

impl<T> Neighborhood<T> {
    pub fn new(up: Weak<T>, right: Weak<T>, down: Weak<T>, left: Weak<T>) -> Self {
        Neighborhood(vec![up, right, down, left])
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
