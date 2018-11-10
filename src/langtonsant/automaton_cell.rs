use enums::pos_2d::Pos2D;

trait AutomatonCell {
    fn pos(&self) -> Pos2D;
    fn find_cell(pos: &Pos2D) -> &Self;
}