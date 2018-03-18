use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
mod ids_tests;

macro_rules! reg_priv {
    ($s:expr, $name:expr, $map:expr) => {{
        if $s.contains($name) {
            $map[&$name]
        } else {
            $s.names.insert($name);
            let id = $s.first_free.fetch_add(1, Ordering::SeqCst);
            $map.insert($name, id);
            id
        }
    }};
}

pub type ValueId = usize;
pub type CellTypeId = usize;

macro_rules! reg {
    ($s:expr, $name:expr,values) => {
        reg_priv!($s, $name, $s.values) as ValueId
    };
    ($s:expr, $name:expr,cell_types) => {
        reg_priv!($s, $name, $s.cell_types) as CellTypeId
    };
}

#[derive(Debug, Default)]
pub struct Identifiers<S>
where
    S: AsRef<str> + Eq + Hash + Copy + Debug,
{
    first_free: AtomicUsize,
    names: HashSet<S>,
    values: HashMap<S, ValueId>,
    cell_types: HashMap<S, ValueId>,
}

pub static NO_ID: usize = 0;

impl<S> Identifiers<S>
where
    S: AsRef<str> + Eq + Hash + Copy + Debug,
{
    pub fn default() -> Self {
        Identifiers {
            first_free: AtomicUsize::new(1),
            names: HashSet::new(),
            values: HashMap::new(),
            cell_types: HashMap::new(),
        }
    }

    #[inline]
    fn contains(&self, name: S) -> bool {
        self.names.contains(&name)
    }

    #[inline]
    fn contains_type(&self, id: usize, map: &HashMap<S, usize>) -> bool {
        map.values().any(|&v| v == id)
    }

    #[inline]
    fn get(&self, name: &S, map: &HashMap<S, usize>) -> Option<usize> {
        map.get(name).cloned()
    }

    pub fn reg_value(&mut self, name: S) -> ValueId {
        reg!(self, name, values)
    }

    pub fn contains_value(&self, id: ValueId) -> bool {
        self.contains_type(id, &self.values)
    }

    pub fn get_value(&self, name: S) -> Option<ValueId> {
        self.get(&name, &self.values)
    }

    pub fn reg_cell_type(&mut self, name: S) -> CellTypeId {
        reg!(self, name, cell_types)
    }

    pub fn contains_cell_type(&self, id: CellTypeId) -> bool {
        self.contains_type(id, &self.cell_types)
    }

    pub fn get_cell_type(&self, name: S) -> Option<CellTypeId> {
        self.get(&name, &self.cell_types)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
