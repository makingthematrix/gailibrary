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
