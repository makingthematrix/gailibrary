use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ValueId(usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CellTypeId(usize);

#[derive(Debug, Default)]
pub struct Identifiers<S>
where
    S: AsRef<str> + Eq + Hash + Copy + Debug,
{
    first_free: AtomicUsize,
    pub names: HashSet<S>,
    pub values: HashMap<S, ValueId>,
    pub cell_types: HashMap<S, CellTypeId>,
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
    pub fn contains(&self, name: S) -> bool {
        self.names.contains(&name)
    }

    pub fn reg_value(&mut self, name: S) -> ValueId {
        if self.contains(name) {
            self.values[&name]
        } else {
            self.names.insert(name);
            let id = ValueId(self.first_free.fetch_add(1, Ordering::SeqCst));
            self.values.insert(name, id);
            id
        }
    }

    pub fn contains_value(&self, id: ValueId) -> bool {
        self.values.values().any(|&v| v == id)
    }

    pub fn get_value(&self, name: S) -> Option<ValueId> {
        self.values.get(&name).cloned()
    }

    pub fn reg_cell_type(&mut self, name: S) -> CellTypeId {
        if self.contains(name) {
            self.cell_types[&name]
        } else {
            self.names.insert(name);
            let id = CellTypeId(self.first_free.fetch_add(1, Ordering::SeqCst));
            self.cell_types.insert(name, id);
            id
        }
    }

    pub fn contains_cell_type(&self, id: CellTypeId) -> bool {
        self.cell_types.values().any(|&v| v == id)
    }

    pub fn get_cell_type(&self, name: S) -> Option<CellTypeId> {
        self.cell_types.get(&name).cloned()
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
