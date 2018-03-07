use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::hash::Hash;
use std::fmt::Debug;

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
    }}
}

pub type ValueId = usize;
pub type CellTypeId = usize;

macro_rules! reg {
    ($s:expr, $name:expr, values) => (reg_priv!($s, $name, $s.values) as ValueId);
    ($s:expr, $name:expr, cell_types) => (reg_priv!($s, $name, $s.cell_types) as CellTypeId);
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
            .named("health")
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
        assert_that(&ids.is_empty()).named("is empty").is_false();
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

    #[test]
    fn should_register_two_cell_type_ids() {
        let mut ids = Identifiers::default();
        let player = ids.reg_cell_type("player");
        let npc = ids.reg_cell_type("npc");
        assert_that(&ids.contains("player")).is_true();
        assert_that(&ids.contains("npc")).is_true();
        assert_that(&ids.contains("wall")).is_false();
        assert_ne!(player, npc);
    }

    #[test]
    fn should_not_mix_types() {
        let mut ids = Identifiers::default();
        let player = ids.reg_cell_type("player");
        let npc = ids.reg_cell_type("npc");
        let health = ids.reg_value("health");
        let ammo = ids.reg_value("ammo");

        println!("names: {:?}", ids.names);
        println!("values: {:?}", ids.values);
        println!("cell types: {:?}", ids.cell_types);

        assert_that(&ids.len()).named("has size").is_equal_to(&4);

        assert_that(&ids.get_cell_type("player")).is_equal_to(&Some(player));
        assert_that(&ids.get_value("player")).is_equal_to(&None);

        assert_that(&ids.get_cell_type("npc")).is_equal_to(&Some(npc));
        assert_that(&ids.get_value("npc")).is_equal_to(&None);

        assert_that(&ids.get_cell_type("health")).is_equal_to(&None);
        assert_that(&ids.get_value("health")).is_equal_to(&Some(health));

        assert_that(&ids.get_cell_type("ammo")).is_equal_to(&None);
        assert_that(&ids.get_value("ammo")).is_equal_to(&Some(ammo));
    }
}
