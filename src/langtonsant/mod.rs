pub mod arena;
pub mod grid;
pub mod langtons_ant;
pub mod neighborhood;
pub mod visualisation;

pub mod automaton_cell;

use langtonsant::arena::Arena;
use langtonsant::grid::*;
use langtonsant::langtons_ant::LangtonsAnt;
use langtonsant::visualisation::Visualisation;
use std::rc::Rc;

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
        let id = x * self.grid.dim() + y;
        self.arena = self.arena.insert(id, &ant);
    }
}

pub fn langtons_ant() {
    let mut anthill = AntHill::new(20);
    anthill.insert_ant(10, 10);

    println!("---");
    anthill.grid.print();

    for _i in 0..100 {
        anthill.update();
        println!("---");
        anthill.grid.print();
    }
}
