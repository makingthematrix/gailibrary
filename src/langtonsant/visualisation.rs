use std::iter::FromIterator;

pub trait Visualisation {
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
