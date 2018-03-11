use cgmath::{Deg, Point3, Rad, Vector3};
use cgmath::prelude::*;

use float_cmp::ApproxEqUlps;

use std::ops::{Add, Deref, Mul, Sub};
use std::ptr;

/// A thin wrapper over `cgmath::Point3<f64>` using the Newtype idiom.
///
/// The idea is to give the user an easy way to convert between  whatever they use in the physics'
/// engine and GAI, and give them a few most usable utility methods, ie. computing the distance,
/// the equality of position, the direction from two positions, and the angle between positions and
/// the current point of view. All the other methods can be accessed by dereferencing.
/// Everything here is likely to change a few times before stabilization.

#[derive(Debug, Clone, Copy, From, Into)]
pub struct Position(Point3<f64>);

pub static CENTER: Position = Position(Point3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
});

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position(Point3::new(x, y, z))
    }

    pub fn new2(x: f64, y: f64) -> Self {
        Position(Point3::new(x, y, 0.0))
    }

    #[inline]
    pub fn distance(&self, other: &Position) -> f64 {
        self.0.distance(other.0)
    }

    #[inline]
    pub fn direction_to(&self, other: &Position) -> Direction {
        Direction((self.0 - other.0).normalize())
    }
}

impl<'a> From<&'a [f64]> for Position {
    fn from(p: &'a [f64]) -> Self {
        let x = p[0];
        let y = p[1];
        let z = if p.len() > 2 { p[2] } else { 0.0 };
        Position::new(x, y, z)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.distance(other).approx_eq_ulps(&0.0, 2)
    }
}

impl Eq for Position {}

impl Deref for Position {
    type Target = Point3<f64>;

    fn deref(&self) -> &Point3<f64> {
        &self.0
    }
}

impl Add<Vector3<f64>> for Position {
    type Output = Position;

    fn add(self, other: Vector3<f64>) -> Position {
        Position(self.0 + other)
    }
}

impl Sub<Position> for Position {
    type Output = Vector3<f64>;

    fn sub(self, other: Position) -> Vector3<f64> {
        self.0 - other.0
    }
}

/// A thin wrapper over `cgmath::Vector3<f64>`
#[derive(Debug, Clone, Copy)]
pub struct Direction(Vector3<f64>);

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Direction(Vector3::new(x, y, z).normalize())
    }

    pub fn new2(x: f64, y: f64) -> Self {
        Direction(Vector3::new(x, y, 0.0).normalize())
    }

    #[inline]
    pub fn angle(&self, other: &Direction) -> Deg<f64> {
        Rad::acos(self.0.dot(other.0)).into()
    }
}

impl From<Vector3<f64>> for Direction {
    fn from(v: Vector3<f64>) -> Self {
        Direction(v.normalize())
    }
}

impl<'a> From<&'a [f64]> for Direction {
    fn from(p: &'a [f64]) -> Self {
        let x = p[0];
        let y = p[1];
        let z = if p.len() > 2 { p[2] } else { 0.0 };
        Direction::new(x, y, z)
    }
}

impl From<Position> for Direction {
    fn from(p: Position) -> Self {
        (p - CENTER).into()
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self.angle(other).0.approx_eq_ulps(&0.0, 2)
    }
}

impl Eq for Direction {}

impl Deref for Direction {
    type Target = Vector3<f64>;

    fn deref(&self) -> &Vector3<f64> {
        &self.0
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, v: Direction) -> Direction {
        Direction((self.0 + v.0).normalize())
    }
}

impl Sub<Direction> for Direction {
    type Output = Direction;

    fn sub(self, v: Direction) -> Direction {
        Direction((self.0 - v.0).normalize())
    }
}

impl Mul<Direction> for Direction {
    type Output = f64;

    fn mul(self, v: Direction) -> f64 {
        self.0.dot(v.0)
    }
}

#[inline]
pub fn angle_between(your_pos: Position, your_pov: Direction, other_pos: Position) -> Deg<f64> {
    your_pov.angle(&(other_pos - your_pos).into())
}

#[derive(Debug, Clone, Copy, From, Into)]
pub struct Coeff(f64);

pub static COEFF_ZERO: Coeff = Coeff(0.0);
pub static COEFF_ONE: Coeff = Coeff(1.0);

impl Coeff {
    pub fn default() -> Self {
        COEFF_ZERO
    }

    pub fn new(c: f64) -> Self {
        debug_assert!(!(c < 0.0 || c > 1.0));
        if c.approx_eq_ulps(&0.0, 2) {
            COEFF_ZERO
        } else if c.approx_eq_ulps(&1.0, 2) {
            COEFF_ONE
        } else {
            Coeff(c)
        }
    }

    pub fn normalize(vec: &[f64]) -> Vec<Coeff> {
        let sum: f64 = vec.iter().sum();
        vec.iter().map(|&v| Coeff::new(v / sum)).collect()
    }
}

impl PartialEq for Coeff {
    fn eq(&self, other: &Coeff) -> bool {
        ptr::eq(&self, &other) || self.0.approx_eq_ulps(&other.0, 2)
    }
}

impl Eq for Coeff {}

use std::iter::Sum;
impl Sum<Coeff> for Coeff {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(COEFF_ZERO, Add::add)
    }
}

#[allow(suspicious_arithmetic_impl)]
impl Add<Coeff> for Coeff {
    type Output = Coeff;

    fn add(self, c: Coeff) -> Coeff {
        let t = self.0 + c.0;
        if t > 1.0 {
            COEFF_ONE
        } else {
            Coeff::new(t)
        }
    }
}

#[allow(suspicious_arithmetic_impl)]
impl Sub<Coeff> for Coeff {
    type Output = Coeff;

    fn sub(self, c: Coeff) -> Coeff {
        let t = self.0 - c.0;
        if t < 0.0 {
            COEFF_ZERO
        } else {
            Coeff::new(t)
        }
    }
}

impl Mul<Coeff> for Coeff {
    type Output = Coeff;

    fn mul(self, c: Coeff) -> Coeff {
        Coeff::new(self.0 * c.0)
    }
}

#[cfg(test)]
mod math_tests {
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
