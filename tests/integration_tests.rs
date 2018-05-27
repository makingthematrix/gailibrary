#![feature(test)]
#![feature(const_fn)]

extern crate gailibrary;

use gailibrary::cell::*;
use gailibrary::enums::white_black::*;
use gailibrary::enums::dir_2d::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LangtonsAnt {
    color: WhiteBlack,
    dir: Dir2D,
    pos: (usize, usize),
}

impl LangtonsAnt {
    const COLOR_ID: ValueId<WhiteBlack> = new_id::<WhiteBlack>(1);
    const DIR_ID: ValueId<Dir2D> = new_id::<Dir2D>(2);

    pub fn new(x: usize, y: usize) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: Dir2D::None,
            pos: (x, y),
        }
    }

    pub fn update_color<'a>(cell: &LangtonsAnt, _nd: &AntNeighborhood<'a>) -> WhiteBlack {
        if cell.dir == Dir2D::None { cell.color } else { cell.color.toggle() }
    }

    pub fn update_dir<'a>(cell: &LangtonsAnt, nd: &AntNeighborhood<'a>) -> Dir2D {
        if cell.dir == Dir2D:: None {
            if let Some(ant_dir) = Dir2D::iter().find(|&d| nd.get_if(d as usize, |c| c.dir == d.turn_around()).is_some()) {
                if cell.color == WhiteBlack::White {
                    ant_dir.turn_left()
                } else {
                    ant_dir.turn_right()
                }
            } else {
                Dir2D:: None
            }
        } else {
            Dir2D::None
        }
    }

}

pub struct Cell<'a, T: 'a> {
    data: T,
    graph: &'a CellGraph<'a, T>,
}

trait CellData<T> {
    fn get(&self, id: ValueId<T>) -> Option<T>;
    fn set<F>(&mut self, id: ValueId<T>, update: F) where F: Fn(&Self) -> T;
}

impl CellData<WhiteBlack> for LangtonsAnt {
    fn get(&self, id: ValueId<WhiteBlack>) -> Option<WhiteBlack> {
        if id == LangtonsAnt::COLOR_ID {
            Some(self.color)
        } else {
            None
        }
    }

    fn set<F>(&mut self, id: ValueId<WhiteBlack>, update: F) where F: Fn(&Self) -> WhiteBlack {
        if id == LangtonsAnt::COLOR_ID {
            self.color = update(self);
        }
    }
}

impl CellData<Dir2D> for LangtonsAnt {
    fn get(&self, id: ValueId<Dir2D>) -> Option<Dir2D> {
        if id == LangtonsAnt::DIR_ID {
            Some(self.dir)
        } else {
            None
        }
    }

    fn set<F>(&mut self, id: ValueId<Dir2D>, update: F) where F: Fn(&Self) -> Dir2D {
        if id == LangtonsAnt::DIR_ID {
            self.dir = update(self);
        }
    }
}

pub struct Grid {
    grid: Vec<LangtonsAnt>,
    dim: usize,
}

impl<'a> Grid {
    fn new(dim: usize) -> Self {
        let mut grid: Vec<LangtonsAnt> = Vec::with_capacity(dim * dim);
        for x in 0..dim {
            for y in 0..dim {
                grid.push(LangtonsAnt::new(x, y));
            }
        }
        Grid { grid, dim }
    }

    #[inline]
    pub fn cell(&'a self, x: usize, y: usize) -> &'a LangtonsAnt {
        &self.grid[x * self.dim + y]
    }

    fn neighborhood(&'a self, c: &LangtonsAnt) -> AntNeighborhood<'a> {
        let x = c.pos.0;
        let y = c.pos.1;
        let last = self.dim - 1;
        let up = if y == 0 {
            self.cell(x, last)
        } else {
            self.cell(x, y - 1)
        };
        let right = if x == last {
            self.cell(0, y)
        } else {
            self.cell(x + 1, y)
        };
        let down = if y == last {
            self.cell(x, 0)
        } else {
            self.cell(x, y + 1)
        };
        let left = if x == 0 {
            self.cell(last, y)
        } else {
            self.cell(x - 1, y)
        };

        AntNeighborhood::new(up, right, down, left)
    }

    fn update_cell(&self, cell: &LangtonsAnt) -> LangtonsAnt {
        let mut new_cell = *cell;
        let nd = self.neighborhood(cell);
        new_cell.set(LangtonsAnt::COLOR_ID, |c: &LangtonsAnt| LangtonsAnt::update_color(c, &nd));
        new_cell.set(LangtonsAnt::DIR_ID, |c: &LangtonsAnt| LangtonsAnt::update_dir(c, &nd));

        if new_cell != *cell {
            println!("{:?} -> {:?}", cell, new_cell);
        }

        new_cell
    }

    fn update(&self) -> Self {
        let grid: Vec<LangtonsAnt> = self.grid.iter().map(|c| self.update_cell(c)).collect();
        Grid { grid, dim: self.dim }
    }
}

pub struct AntGraph<'a> {
    grid: &'a Grid
}

trait CellGraph<'a, T: 'a> {
    fn all_near(&self, c: &'a T) -> Vec<&'a T> where Self: Sized;

    fn first_near_if<F>(&self, cell: &'a T, pred: F) -> Option<&'a T> where Self: Sized, F: Fn(&'a T) -> bool {
        let nd = self.all_near(cell);
        nd.iter().find(|&c| pred(*c)).map(|&c| c)
    }

    fn all_near_if<F>(&self, cell: &'a T, pred: F) -> Vec<&'a T> where Self: Sized, F: Fn(&'a T) -> bool {
        let nd = self.all_near(cell);
        nd.iter().filter_map(|&c| if pred(c) { Some(c) } else { None }).collect()
    }

    fn all_near_ordered<F>(&self, cell: &'a T, pred: F) -> Vec<Option<&'a T>> where Self: Sized, F: Fn(&'a T) -> bool {
        let nd = self.all_near(cell);
        nd.iter().map(|&c| if pred(c) { Some(c) } else { None }).collect()
    }
}

impl<'a> CellGraph<'a, LangtonsAnt> for AntGraph<'a> {
    #[inline]
    fn all_near(&self, c: &'a LangtonsAnt) -> Vec<&'a LangtonsAnt> where Self: Sized {
        let x = c.pos.0;
        let y = c.pos.1;
        let last = self.grid.dim - 1;
        let up = if y == 0 {
            self.grid.cell(x, last)
        } else {
            self.grid.cell(x, y - 1)
        };
        let right = if x == last {
            self.grid.cell(0, y)
        } else {
            self.grid.cell(x + 1, y)
        };
        let down = if y == last {
            self.grid.cell(x, 0)
        } else {
            self.grid.cell(x, y + 1)
        };
        let left = if x == 0 {
            self.grid.cell(last, y)
        } else {
            self.grid.cell(x - 1, y)
        };

        vec![up, right, down, left]
    }
}


pub struct AntNeighborhood<'a> {
    nd: Vec<&'a LangtonsAnt>,
}

impl<'a> AntNeighborhood<'a> {
    pub fn new(up: &'a LangtonsAnt, right: &'a LangtonsAnt, down: &'a LangtonsAnt, left: &'a LangtonsAnt) -> AntNeighborhood<'a> {
        AntNeighborhood { nd: vec![up, right, down, left] }
    }

    pub fn get_if<F>(&self, index: usize, pred: F) -> Option<&LangtonsAnt> where Self: Sized, F: Fn(&'a LangtonsAnt) -> bool, {
        if index < self.nd.len() && pred(self.nd[index]) { Some(self.nd[index]) } else { None }
    }
}

trait Neighborhood<'a, T: 'a> {
    fn find_first<F>(&self, pred: F) -> Option<&T> where Self: Sized, F: Fn(&'a T) -> bool;
    fn find_all<F>(&self, pred: F) -> Vec<&T> where Self: Sized, F: Fn(&'a T) -> bool;
}

impl<'a> Neighborhood<'a, LangtonsAnt> for AntNeighborhood<'a> {
    fn find_first<F>(&self, pred: F) -> Option<&LangtonsAnt> where Self: Sized, F: Fn(&'a LangtonsAnt) -> bool {
        self.nd.iter().find(|&c| pred(*c)).map(|&c| c)
    }

    fn find_all<F>(&self, pred: F) -> Vec<&LangtonsAnt> where Self: Sized, F: Fn(&'a LangtonsAnt) -> bool {
        self.nd.iter().filter_map(|&c| if pred(c) { Some(c) } else { None }).collect()
    }
}

#[test]
fn langtons_ant() {
    let grid = Grid::new(100);
    let _new_grid = grid.update();
}
