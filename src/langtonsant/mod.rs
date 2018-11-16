pub mod langtons_ant;
pub mod lant;
pub mod visualisation;

use engine::arena::Arena;
use engine::automaton::Automaton;
use engine::grid::*;
use enums::pos_2d::Pos2D;
use langtonsant::langtons_ant::LangtonsAnt;
use langtonsant::lant::LAnt;
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

    pub fn insert_ant(&mut self, pos: &Pos2D) {
        let ant = Rc::new(LangtonsAnt::new_ant(pos, Rc::downgrade(&self.grid)));

        let id = self.grid.insert(pos, Rc::downgrade(&ant));
        self.arena = self.arena.insert(id, &ant);
    }
}

pub fn langtons_ant() {
    let mut anthill = AntHill::new(20);
    anthill.insert_ant(&Pos2D::new(10, 10));

    println!("---");
    anthill.grid.print();

    for _i in 0..100 {
        anthill.update();
        println!("---");
        anthill.grid.print();
    }
}

pub fn langtons_ant2() {
    let mut auto = Automaton::<LAnt>::new(20);
    let ant_pos = Pos2D::new(10, 10);
    auto.change(|board| board.change_one(&ant_pos, |_| LAnt::new_ant(&ant_pos)));

    println!("---");
    auto.0.print();

    for _i in 0..100 {
        auto.update();
        println!("---");
        auto.0.print();
    }
}
