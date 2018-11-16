use engine::board::Neigh;
use enums::pos_2d::Pos2D;

pub trait AutomatonCell: Clone + Copy + PartialEq {
    fn update(&self, neighborhood: &Neigh<Self>) -> Self;

    fn new(pos: &Pos2D) -> Self;
}
