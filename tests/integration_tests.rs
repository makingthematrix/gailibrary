#![feature(test)]
#![feature(const_fn)]

extern crate gailibrary;

use gailibrary::enums::dir_2d::*;
use gailibrary::enums::white_black::*;

use std::fmt;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use std::iter::FromIterator;

use gailibrary::utils::umap::*;

#[derive(Clone)]
pub struct LangtonsAnt {
    color: WhiteBlack,
    dir: Dir2D,
    pos: (usize, usize),
    grid: Weak<Grid<LangtonsAnt>>,
}

impl fmt::Debug for LangtonsAnt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LangtonsAnt(color: {}, dir: {}, pos: ({}, {}))",
            self.color, self.dir, self.pos.0, self.pos.1
        )
    }
}

impl PartialEq for LangtonsAnt {
    fn eq(&self, other: &LangtonsAnt) -> bool {
        self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1
    }
}

impl LangtonsAnt {
    pub fn new(x: usize, y: usize, grid: Weak<Grid<LangtonsAnt>>) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: Dir2D::None,
            pos: (x, y),
            grid: grid,
        }
    }

    pub fn new_ant(x: usize, y: usize, grid: Weak<Grid<LangtonsAnt>>) -> Self {
        LangtonsAnt {
            color: WhiteBlack::White,
            dir: Dir2D::Up,
            pos: (x, y),
            grid,
        }
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.pos.0
    }

    #[inline]
    pub fn y(&self) -> usize {
        self.pos.1
    }

    #[inline]
    fn grid(&self) -> Rc<Grid<LangtonsAnt>> {
        self.grid.upgrade().unwrap()
    }

    fn update_color(&self) -> WhiteBlack {
        if self.dir == Dir2D::None {
            self.color
        } else {
            self.color.toggle()
        }
    }

    pub fn update_dir(&self) -> Dir2D {
        if self.dir == Dir2D::None {
            if let Some((.., ant_dir)) = self
                .grid()
                .all_near(self)
                .find(|c, d| c.dir == d.turn_around())
            {
                if self.color == WhiteBlack::White {
                    ant_dir.turn_left()
                } else {
                    ant_dir.turn_right()
                }
            } else {
                Dir2D::None
            }
        } else {
            Dir2D::None
        }
    }

    pub fn update(&self) -> Self {
        LangtonsAnt {
            color: self.update_color(),
            dir: self.update_dir(),
            pos: (self.pos.0, self.pos.1),
            grid: self.grid.clone(),
        }
    }
}

pub struct Neighborhood<T>(Vec<Weak<T>>);

impl Neighborhood<LangtonsAnt> {
    pub fn new(
        up: Weak<LangtonsAnt>,
        right: Weak<LangtonsAnt>,
        down: Weak<LangtonsAnt>,
        left: Weak<LangtonsAnt>,
    ) -> Self {
        Neighborhood(vec![up, right, down, left])
    }

    pub fn find<F>(&self, pred: F) -> Option<(Weak<LangtonsAnt>, Dir2D)>
    where
        F: Fn(&LangtonsAnt, Dir2D) -> bool,
    {
        let dir = Dir2D::iter().find(|&dir| {
            let vec = &self.0;
            let index = dir as usize;

            let link = &vec[index];
            if let Some(cell) = link.upgrade() {
                pred(&cell, dir)
            } else {
                false
            }
        });

        dir.map(|d| (self.0[d as usize].clone(), d))
    }
}

impl fmt::Debug for Neighborhood<LangtonsAnt> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        let u = vec[0].upgrade().unwrap();
        let r = vec[1].upgrade().unwrap();
        let d = vec[2].upgrade().unwrap();
        let l = vec[3].upgrade().unwrap();

        write!(
            f,
            "Neighborhood<LangtonsAnt>(\nUP   : {:?},\nRIGHT: {:?},\nDOWN : {:?},\nLEFT : {:?})",
            u, r, d, l
        )
    }
}

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

pub struct Arena<T>(UMap<Rc<T>>);

impl Arena<LangtonsAnt> {
    pub fn new(capacity: usize) -> Self {
        Arena(UMap::<Rc<LangtonsAnt>>::with_capacity(capacity))
    }

    pub fn init(&mut self, grid: &Rc<Grid<LangtonsAnt>>) {
        let dim = grid.dim;
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

pub struct AntHill {
    arena: Arena<LangtonsAnt>,
    grid: Rc<Grid<LangtonsAnt>>,
}

impl AntHill {
    pub fn new(dim: usize) -> Self {
        let mut arena = Arena::<LangtonsAnt>::new(dim * dim);
        let grid = Rc::new(Grid::new(dim));

        arena.init(&grid);
        grid.update(&arena);

        AntHill { arena, grid }
    }

    pub fn update(&mut self) {
        let arena = self.arena.update();
        self.grid.update(&arena);
        self.arena = arena;
    }

    pub fn insert_ant(&mut self, x: usize, y: usize) {
        let ant = Rc::new(LangtonsAnt::new_ant(x, y, Rc::downgrade(&self.grid)));

        self.grid.insert(x, y, Rc::downgrade(&ant));
        let id = x * self.grid.dim + y;
        self.arena = self.arena.insert(id, &ant);
    }
}

trait Visualisation {
    fn grid(&self) -> (Vec<char>, usize);

    fn print(&self) {
        let (gr, dim) = self.grid();
        for i in 0..dim {
            let a = i * dim;
            let b = a + dim - 1;
            println!("{}", String::from_iter(gr[a..b].iter()));
        }
    }
}

impl Visualisation for Grid<LangtonsAnt> {
    fn grid(&self) -> (Vec<char>, usize) {
        fn to_char(cell: &Option<Weak<LangtonsAnt>>) -> char {
            if let Some(ref rf) = cell {
                if let Some(ant) = rf.upgrade() {
                    match ant.dir {
                        Dir2D::None => if ant.color == WhiteBlack::White {
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

        let rf = self.vec.borrow();
        let res: Vec<char> = rf.iter().map(|c| to_char(c)).collect();
        (res, self.dim)
    }
}

#[test]
fn langtons_ant() {
    let mut anthill = AntHill::new(10);
    anthill.insert_ant(5, 5);

    println!("---");
    anthill.grid.print();

    for _i in 0..10 {
        anthill.update();
        println!("---");
        anthill.grid.print();
    }
}
