use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::hash::Hash;

pub type ValueId = usize;
pub type CellTypeId = usize;

#[derive(Debug, Default)]
pub struct Identifiers<S>
where
    S: AsRef<str> + Eq + Hash + Copy,
{
    first_free: AtomicUsize,
    names: HashSet<S>,
    values: HashMap<S, ValueId>,
    cell_types: HashMap<S, ValueId>,
}

pub static NO_ID: usize = 0;

// TODO: use macros instead of inline functions (especially for 'reg...')
impl<S> Identifiers<S>
where
    S: AsRef<str> + Eq + Hash + Copy,
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
    fn reg_name(&mut self, name: S) -> Option<usize> {
        if self.contains(name) {
            None
        } else {
            self.names.insert(name);
            Some(self.first_free.fetch_add(1, Ordering::SeqCst))
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
        match self.reg_name(name) {
            Some(id) => {
                self.values.insert(name, id);
                id
            }
            None => self.values[&name],
        }
    }

    pub fn contains_value(&self, id: ValueId) -> bool {
        self.contains_type(id, &self.values)
    }

    pub fn get_value(&self, name: S) -> Option<ValueId> {
        self.get(&name, &self.values)
    }

    pub fn reg_cell_type(&mut self, name: S) -> CellTypeId {
        match self.reg_name(name) {
            Some(id) => {
                self.cell_types.insert(name, id);
                id
            }
            None => self.cell_types[&name],
        }
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

#[cfg(test)]
mod ids_tests {
    use ids::*;
    use spectral::prelude::*;

    #[test]
    fn should_register_value_id() {
        let mut ids = Identifiers::default();
        let health = ids.reg_value("health");
        assert_that(&ids.contains("health"))
            .named("has health")
            .is_true();
        assert_that(&ids.get_value("health"))
            .named(&"health")
            .is_equal_to(&Some(health));
    }

    #[test]
    fn should_register_two_value_ids() {
        let mut ids = Identifiers::default();
        let health = ids.reg_value("health");
        let ammo = ids.reg_value("ammo");
        assert_that(&ids.contains("health")).is_true();
        assert_that(&ids.contains("ammo")).is_true();
        assert_that(&ids.contains("pos")).is_false();
        assert_ne!(health, ammo);
    }

    #[test]
    fn should_not_allow_two_identical_names() {
        let mut ids = Identifiers::default();
        let health = ids.reg_value("health");
        assert_that(&ids.reg_value("health")).is_equal_to(&health);
        assert_eq!(&ids.len(), &1);
    }

    quickcheck! {
        fn populate_identifiers(v: Vec<String>) -> bool {
            let mut ids = Identifiers::default();
            let results: Vec<ValueId> = v.iter().filter_map(|s| {
                if !ids.contains(s) {
                    Some(ids.reg_value(s))
                } else {
                    None
                }
            }).collect();

            results.iter().all(|&id| ids.contains_value(id))
        }
    }

}
