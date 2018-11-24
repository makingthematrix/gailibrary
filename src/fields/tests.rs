#[cfg(test)]
mod tests {

    use math::*;
    use std::ptr::eq;
    use spectral::prelude::*;
    pub use cgmath::*;

    #[test]
    fn should_compute_distance() {
        let p1 = Position::new(0.5, 0.5, 0.0);
        let p2 = Position::new(0.0, 1.0, 0.0);
        assert_ulps_eq!(p1.distance(&p2), ((0.5 * 0.5 + 0.5 * 0.5) as f64).sqrt());
    }

    #[test]
    fn should_points_be_equal() {
        let p1 = Position::new(0.2, 0.2, 0.2);
        let p2 = Position::new(0.2, 0.2, 0.2);

        assert_that(&eq(&p1, &p2)).is_false();
        assert_that(&p1).is_equal_to(&p2);
        assert_that(&p1.distance(&p2)).is_equal_to(0.0);
    }

    #[test]
    fn should_compute_orientation() {
        let v1 = Direction::new(0.5, 0.5, 0.0); // where the npc is looking
        let v2 = Direction::new(0.0, 1.0, 0.0); // where the npc should be looking

        let deg = v1.angle(&v2);
        assert_ulps_eq!(deg, Deg(45.0)); // what are you looking at, npc?!
    }

    #[test]
    fn should_compute_position_angle() {
        let p1 = Position::new(0.5, 0.5, 0.0); // the player's position
        let v1 = Direction::new(0.0, 1.0, 0.0); // where the player is looking
        let p2 = Position::new(0.0, 1.0, 0.0); // the npc's position

        let deg = angle_between(p1, v1, p2);
        assert_ulps_eq!(deg, Deg(45.0)); // guess the npc is safe
    }

    #[test]
    fn should_create_coeffs() {
        assert_that(&Coeff::new(0.0)).is_equal_to(&COEFF_ZERO);
        assert_that(&Coeff::new(1.0)).is_equal_to(&COEFF_ONE);
        assert_that(&Coeff::new(0.5)).is_not_equal_to(&COEFF_ZERO);
        assert_that(&Coeff::new(0.5)).is_not_equal_to(&COEFF_ONE);
    }

    #[test]
    fn should_add_coeffs() {
        let c1 = Coeff::new(0.2);
        let c2 = Coeff::new(0.5);
        assert_that(&(c1 + c2)).is_equal_to(&Coeff::new(0.7));

        let c3 = Coeff::new(0.8);
        assert_that(&(c1 + c3)).is_equal_to(&COEFF_ONE);
    }

    #[test]
    fn should_substract_coeffs() {
        let c1 = Coeff::new(0.7);
        let c2 = Coeff::new(0.5);
        assert_that(&(c1 - c2)).is_equal_to(&Coeff::new(0.2));
    }

    #[test]
    fn should_multiply_coeffs() {
        let c1 = Coeff::new(0.5);
        let c2 = Coeff::new(0.5);
        assert_that(&(c1 * c2)).is_equal_to(&Coeff::new(0.25));
    }

    #[test]
    fn should_normalize() {
        let v = vec![1.0, 3.0, 4.0, 2.0];
        let v2 = Coeff::normalize(&v);
        assert_that(&v).has_length(v2.len());
        // TODO: Learn why `iter()` is not compiling and `into_iter()` is ok
        let sum: Coeff = v2.clone().into_iter().sum();
        assert_that(&sum).is_equal_to(&COEFF_ONE);

        assert_that(&v2[0]).is_equal_to(Coeff::new(0.1));
        assert_that(&v2[1]).is_equal_to(Coeff::new(0.3));
        assert_that(&v2[2]).is_equal_to(Coeff::new(0.4));
        assert_that(&v2[3]).is_equal_to(Coeff::new(0.2));
    }
}