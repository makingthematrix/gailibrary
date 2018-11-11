use cgmath::prelude::*;
use cgmath::{Deg, Point3, Rad, Vector3};

use float_cmp::ApproxEqUlps;

use std::ops::{Add, Deref, Mul, Neg, Sub};
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

    fn add(self, v: Vector3<f64>) -> Position {
        Position(self.0 + v)
    }
}

// Not intuitive: Direction is not really a vector. But it can be made into a vector by multiplying
// by 1.0. I want to avoid code like `position + (direction * 1.0)`, where `* 1.0` is used only for
// type casting.
impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, d: Direction) -> Position {
        self + *d
    }
}

impl Sub<Position> for Position {
    type Output = Vector3<f64>;

    fn sub(self, other: Position) -> Vector3<f64> {
        self.0 - other.0
    }
}

impl Sub<Vector3<f64>> for Position {
    type Output = Position;

    fn sub(self, v: Vector3<f64>) -> Position {
        Position(self.0 - v)
    }
}

impl Sub<Direction> for Position {
    type Output = Position;

    fn sub(self, d: Direction) -> Position {
        self - *d
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Position {
        Position::new(
            CENTER.0.x - self.0.x,
            CENTER.0.y - self.0.y,
            CENTER.0.z - self.0.z,
        )
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

    #[inline]
    pub fn cross(&self, other: &Direction) -> Vector3<f64> {
        self.0.cross(other.0)
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
        self.0.x.approx_eq_ulps(&other.0.x, 2)
            && self.0.y.approx_eq_ulps(&other.0.y, 2)
            && self.0.z.approx_eq_ulps(&other.0.z, 2)
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

impl Mul<f64> for Direction {
    type Output = Vector3<f64>;

    fn mul(self, v: f64) -> Vector3<f64> {
        Vector3::new(self.0.x * v, self.0.y * v, self.0.z * v)
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        Direction(-self.0)
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
