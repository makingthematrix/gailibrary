#[cfg(test)]
mod cell_tests {
    use cell::*;
    use math::*;
    use spectral::prelude::*;

    #[test]
    fn should_compare() {
        let v1 = Value::Position(Position::new(0.5, 0.5, 0.0));
        let v2 = Value::Position(Position::new(0.0, 1.0, 0.0));

        let equal_pos = match (&v1, &v2) {
            (&Value::Position(p1), &Value::Position(p2)) => p1 == p2,
            (_, _) => panic!("Invalid types comparison"),
        };

        assert_that(&equal_pos).is_false();

        let equal_values = v1 == v2;

        assert_that(&equal_values).is_false();
    }

    #[test]
    fn should_add_dir() {
        let d1 = Value::Direction(Direction::new(1.0, 0.0, 0.0));
        let d2 = d1 + Value::Direction(Direction::new(0.0, 0.0, 1.0));
        assert_that(&d2).is_equal_to(&Value::Direction(Direction::new(0.5, 0.0, 0.5)));
    }

    #[test]
    fn should_add_pos_to_vector() {
        let p1 = Value::Position(Position::new(1.0, 0.0, 0.0));
        let v1 = Vector3::new(0.0, 0.0, 1.0);
        let p2 = p1 + v1;
        assert_that(&p2).is_equal_to(&Value::Position(Position::new(1.0, 0.0, 1.0)));

        let p3 = p1 + v1 * 0.5;
        assert_that(&p3).is_equal_to(&Value::Position(Position::new(1.0, 0.0, 0.5)));
    }

    // tests for invalid operations?
}
