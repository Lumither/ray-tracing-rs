//! # Scalar
//! in this mod, all scalar types are defined
//! such as fraction, complex number and so on
//! these scalar types are used to make
//!

use std::ops::{Add, Div, Mul, Neg, Sub};

mod fraction;

// pub use fraction::Rational;

pub trait Scalar:
    Add<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Copy
    + Sized
{
}

impl Scalar for f64 {}
impl Scalar for f32 {}
// impl Scalar for Rational {}
