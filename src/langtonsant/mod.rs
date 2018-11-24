pub mod lant;
pub mod rc_langtons_ant;
pub mod visualisation;

use std::rc::Rc;

use engine::automaton::Automaton;
use engine::rc_automaton::RcAutomaton;
use fields::Pos2D;

use langtonsant::lant::LAnt;
use langtonsant::rc_langtons_ant::RcLangtonsAnt;
use langtonsant::visualisation::Visualisation;

pub fn langtons_ant() {
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

pub fn langtons_ant_rc() {
    let mut auto = RcAutomaton::<RcLangtonsAnt>::new(20);
    let ant_pos = Pos2D::new(10, 10);
    let ant = Rc::new(RcLangtonsAnt::new_ant(&ant_pos, auto.get_grid()));
    auto.insert(&ant_pos, &ant);

    println!("---");
    auto.grid.print();

    for _i in 0..100 {
        auto.update();
        println!("---");
        auto.grid.print();
    }
}
