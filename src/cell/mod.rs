use math;
use time;

use cgmath::Vector3;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
mod cell_tests;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
    Position(math::Position),
    Direction(math::Direction),
    Coeff(math::Coeff),
    Time(time::SteadyTime),
    Duration(time::Duration),
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(x1), Value::Int(x2)) => Value::Int(x1 + x2),
            (Value::Int(x1), Value::Float(x2)) => Value::Float(x1 as f64 + x2),
            (Value::Int(_), _) => panic!("Int + X supported only for Ints and Floats"),
            (Value::Float(x1), Value::Float(x2)) => Value::Float(x1 + x2),
            (Value::Float(x1), Value::Int(x2)) => Value::Float(x1 + x2 as f64),
            (Value::Float(_), _) => panic!("Float + X supported only for Ints and Floats"),
            (Value::Position(p), Value::Direction(d)) => Value::Position(p + d),
            (Value::Position(_), _) => {
                panic!("Position + X supported only for Dirs and Vector3<f64>")
            }
            (Value::Direction(x1), Value::Direction(x2)) => Value::Direction(x1 + x2),
            (Value::Direction(_), _) => panic!("Direction + X supported only for Dirs"),
            (Value::Coeff(x1), Value::Coeff(x2)) => Value::Coeff(x1 + x2),
            (Value::Coeff(_), _) => panic!("Coeff + X supported only for Coeffs"),
            (Value::Time(t), Value::Duration(d)) => Value::Time(t + d),
            (Value::Time(_), _) => panic!("Time + X supported only for Duration s"),
            (Value::Duration(x1), Value::Duration(x2)) => Value::Duration(x1 + x2),
            (Value::Duration(_), _) => panic!("Duration + X supported only for Durations"),
        }
    }
}

impl Add<Vector3<f64>> for Value {
    type Output = Value;

    fn add(self, v: Vector3<f64>) -> Value {
        match self {
            Value::Position(p) => Value::Position(p + v),
            _ => panic!("Position + X supported only for Vec3<f64>"),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(x1), Value::Int(x2)) => Value::Int(x1 - x2),
            (Value::Int(x1), Value::Float(x2)) => Value::Float(x1 as f64 - x2),
            (Value::Int(_), _) => panic!("Int - X supported only for Ints and Floats"),
            (Value::Float(x1), Value::Float(x2)) => Value::Float(x1 - x2),
            (Value::Float(x1), Value::Int(x2)) => Value::Float(x1 - x2 as f64),
            (Value::Float(_), _) => panic!("Float - X supported only for Ints and Floats"),
            (Value::Position(p), Value::Direction(d)) => Value::Position(p - d), // `Dir` is treated here as a Vector3<f64>
            (Value::Position(_), _) => {
                panic!("Position + X supported only for Dirs and Vector3<f64>")
            }
            (Value::Direction(x1), Value::Direction(x2)) => Value::Direction(x1 - x2),
            (Value::Direction(_), _) => panic!("Direction - X supported only for Dirs"),
            (Value::Coeff(x1), Value::Coeff(x2)) => Value::Coeff(x1 - x2),
            (Value::Coeff(_), _) => panic!("Coeff - X supported only for Coeffs"),
            (Value::Time(t), Value::Duration(d)) => Value::Time(t - d),
            (Value::Time(_), _) => panic!("Time - X supported only for Durations"),
            (Value::Duration(x1), Value::Duration(x2)) => Value::Duration(x1 - x2),
            (Value::Duration(_), _) => panic!("Duration - X supported only for Durations"),
        }
    }
}

impl Sub<Vector3<f64>> for Value {
    type Output = Value;

    fn sub(self, v: Vector3<f64>) -> Value {
        match self {
            Value::Position(p) => Value::Position(p - v),
            _ => panic!("Position - X supported only for Vec3<f64>"),
        }
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(x1), Value::Int(x2)) => Value::Int(x1 * x2),
            (Value::Int(x1), Value::Float(x2)) => Value::Float(x1 as f64 * x2),
            (Value::Int(_), _) => panic!("Int * X supported only for Ints and Floats"),
            (Value::Float(x1), Value::Float(x2)) => Value::Float(x1 * x2),
            (Value::Float(x1), Value::Int(x2)) => Value::Float(x1 * x2 as f64),
            (Value::Float(_), _) => panic!("Float * X supported only for Ints and Floats"),
            (Value::Position(_), _) => panic!("Position * X is not supported"),
            (Value::Direction(_), _) => panic!("Direction * X is not supported"),
            (Value::Coeff(x1), Value::Coeff(x2)) => Value::Coeff(x1 * x2),
            (Value::Coeff(_), _) => panic!("Coeff * X supported only for Coeffs"),
            (Value::Time(_), _) => panic!("Time * X is not supported"),
            (Value::Duration(_), _) => panic!("Duration * X is not supported"),
        }
    }
}

impl Div<Value> for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(x1), Value::Int(x2)) => Value::Int(x1 / x2),
            (Value::Int(x1), Value::Float(x2)) => Value::Float(x1 as f64 / x2),
            (Value::Int(_), _) => panic!("Int / X supported only for Ints and Floats"),
            (Value::Float(x1), Value::Float(x2)) => Value::Float(x1 / x2),
            (Value::Float(x1), Value::Int(x2)) => Value::Float(x1 / x2 as f64),
            (Value::Float(_), _) => panic!("Float / X supported only for Ints and Floats"),
            (Value::Position(_), _) => panic!("Position / X is not supported"),
            (Value::Direction(_), _) => panic!("Direction / X is not supported"),
            (Value::Coeff(_), _) => panic!("Coeff / X is not supported"),
            (Value::Time(_), _) => panic!("Time / X is not supported"),
            (Value::Duration(_), _) => panic!("Duration / X is not supported"),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Value {
        match self {
            Value::Int(x) => Value::Int(-x),
            Value::Float(x) => Value::Float(-x),
            Value::Position(p) => Value::Position(-p),
            Value::Direction(d) => Value::Direction(-d),
            Value::Coeff(_) => panic!("-Coeff is not supported"),
            Value::Time(_) => panic!("-Time is not supported"),
            Value::Duration(d) => Value::Duration(-d),
        }
    }
}

// new approach

use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ValueId<T> {
    id: usize,
    _marker: PhantomData<T>,
}

impl<T> ValueId<T> {
    pub fn new(id: usize) -> ValueId<T> {
        ValueId::<T> {
            id,
            _marker: PhantomData,
        }
    }
}
