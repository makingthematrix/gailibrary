use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::hash::Hash;

pub type ValueId = usize;

#[derive(Debug, Default)]
pub struct Identifiers<S: AsRef<str> + Eq + Hash + Copy> {
    first_free: AtomicUsize,
    names: HashSet<S>,
    values: HashMap<S, ValueId>,
}

pub static NO_ID: usize = 0;

impl<S: AsRef<str> + Eq + Hash + Copy> Identifiers<S> {
    pub fn default() -> Self {
        Identifiers {
            first_free: AtomicUsize::new(1),
            names: HashSet::new(),
            values: HashMap::new(),
        }
    }

    pub fn reg_value(&mut self, name: S) -> ValueId {
        if self.contains(name) {
            self.values[&name]
        } else {
            self.names.insert(name);
            let id = self.first_free.fetch_add(1, Ordering::SeqCst);
            self.values.insert(name, id);
            id as ValueId
        }
    }

    pub fn contains(&self, name: S) -> bool {
        self.names.contains(&name)
    }

    pub fn contains_value(&self, id: ValueId) -> bool {
        self.values.values().any(|&v| v == id)
    }

    pub fn get_value(&self, name: S) -> Option<ValueId> {
        self.values.get(&name).map(|&id| id as ValueId)
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
