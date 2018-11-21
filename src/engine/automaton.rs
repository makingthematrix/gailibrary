use engine::automaton_cell::AutomatonCell;
use engine::board::Board;

#[derive(Default, Clone)]
pub struct Automaton<C: AutomatonCell>(pub Board<C>);

impl<C: AutomatonCell> Automaton<C> {
    pub fn new(dim: usize) -> Automaton<C> {
        Automaton(Board::<C>::new(dim))
    }

    pub fn update(&mut self) {
        self.0 = self.0.update();
    }

    pub fn change(&mut self, f: impl Fn(&Board<C>) -> Board<C>) {
        self.0 = f(&self.0);
    }
}

impl<C: AutomatonCell> Iterator for Automaton<C> {
    type Item = Board<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.update();
        Some(self.0.clone())
    }
}
