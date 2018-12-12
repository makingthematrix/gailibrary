pub mod langtons_ant;
pub mod rc_langtons_ant;
pub mod visualisation;

use std::rc::Rc;

use crate::engine::automaton::Automaton;
use crate::engine::rc_automaton::RcAutomaton;
use crate::fields::Pos2D;
use crate::langtonsant::langtons_ant::LangtonsAnt;
use crate::langtonsant::rc_langtons_ant::RcLangtonsAnt;
use crate::langtonsant::visualisation::Visualisation;

pub fn langtons_ant(dim: usize, steps: usize) {
    let mut auto = Automaton::<LangtonsAnt>::new(dim);
    let ant_pos = Pos2D::new((dim as i64) / 2, (dim as i64) / 2);
    auto.change(|board| board.change_one(&ant_pos, |_| LangtonsAnt::new_ant(&ant_pos)));

    println!("---");
    auto.0.print();

    for _i in 0..steps {
        auto.update();
        println!("---");
        auto.0.print();
    }
}

pub fn langtons_ant_rc(dim: usize, steps: usize) {
    let mut auto = RcAutomaton::<RcLangtonsAnt>::new(dim);
    let ant_pos = Pos2D::new((dim as i64) / 2, (dim as i64) / 2);
    let ant = Rc::new(RcLangtonsAnt::new_ant(&ant_pos, auto.get_grid()));
    auto.insert(&ant_pos, &ant);

    println!("---");
    auto.grid.print();

    for _i in 0..steps {
        auto.update();
        println!("---");
        auto.grid.print();
    }
}
