//!
//!
//!
//!
//!
//!
use std::ops::{self, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::idx::{IdxVec2d, IdxVec3d, Indexer};

use super::ERROR;

// #[derive(Debug, Clone)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3d(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Vec2d(pub f64, pub f64);

impl Vec3d {
    #[inline]
    pub fn new() -> Vec3d {
        Vec3d(0.0, 0.0, 0.0)
    }

    //------------------------------------------------------------
    // getters
    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }

    // min-max-abs
    #[inline]
    pub fn max_val(&self) -> f64 {
        self.0.max(self.1).max(self.2)
    }

    #[inline]
    pub fn min_val(&self) -> f64 {
        self.0.min(self.1).min(self.2)
    }

    pub fn dominant_axis(&self) -> usize {
        let (x, y, z) = (self.0.abs(), self.1.abs(), self.2.abs());
        [x, y, z]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    pub fn subinant_axis(&self) -> usize {
        let (x, y, z) = (self.0.abs(), self.1.abs(), self.2.abs());
        [x, y, z]
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    pub fn midinant_axis(&self) -> usize {
        let d = self.dominant_axis();
        let s = self.subinant_axis();
        3 - d - s
    }

    //------------------------------------------------------------
    // accessors with calculation
    #[inline]
    pub fn dot(&self, v: &Vec3d) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    #[inline]
    pub fn cross(&self, v: &Vec3d) -> Vec3d {
        Vec3d(
            self.1 * v.2 - v.2 * self.1,
            v.0 * self.2 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    #[inline]
    pub fn det(&self, v1: &Vec3d, v2: &Vec3d) -> f64 {
        return self.0 * (v1.1 * v2.2 - v2.1 * v1.2)
            + self.1 * (v2.0 * v1.2 - v1.0 * v2.2)
            + self.2 * (v1.0 * v2.1 - v2.0 * v1.1);
    }

    #[inline]
    pub fn sum(&self) -> f64 {
        self.0 + self.1 + self.2
    }

    #[inline]
    pub fn abs(&self) -> Vec3d {
        Vec3d(
            self.0.max(-self.0),
            self.1.max(-self.1),
            self.2.max(-self.2),
        )
    }

    #[inline]
    pub fn max_vec(&self, v: &Self) -> Vec3d {
        Vec3d(self.0.max(v.0), self.1.max(v.1), self.2.max(v.2))
    }

    #[inline]
    pub fn min_vec(&self, v: &Self) -> Vec3d {
        Vec3d(self.0.min(v.0), self.1.min(v.1), self.2.min(v.2))
    }

    #[inline]
    pub fn r#box(&self, v1: &Self, v2: &Self) -> f64 {
        Self::dot(&Self::cross(v1, v2), self)
    }

    // get the magnitude
    #[inline]
    pub fn sqr_mag(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    pub fn mag(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    #[inline]
    pub fn cycle_axis(axis: usize, direction: isize) -> usize {
        ((axis as isize + direction) as usize % 3_usize) as usize
    }

    // for the norm values of the vector.
    pub fn l1_norm(&self) -> f64 {
        self.0.max(-self.0) + self.1.max(-self.1) + self.2.max(-self.2)
    }

    #[inline]
    pub fn l2_norm(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    #[inline]
    pub fn linf_norm(&self) -> f64 {
        // self.abs().
        self.abs().max_val()
    }

    //------------------------------------------------------------
    // setters
    pub fn max_set(&mut self, v: &Vec3d) -> &Self {
        self.0 = self.0.max(v.0);
        self.1 = self.1.max(v.1);
        self.2 = self.2.max(v.2);
        self
    }

    pub fn min_set(&mut self, v: &Vec3d) -> &Self {
        self.0 = self.0.min(v.0);
        self.1 = self.1.min(v.1);
        self.2 = self.2.min(v.2);
        self
    }
    //------------------------------------------------------------
    // modifiers

    /// normalize the vector itself (make it become a vector with same direction and magnetude of 1)
    pub fn normalize(&mut self) -> &Self {
        let m = self.mag();
        if m != 0.0 {
            *self = *self / m
        }
        self
    }
}

impl ops::Index<usize> for Vec3d {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("{ERROR}: index out of range"),
        }
    }
}

impl<T> ops::Index<T> for Vec3d
where
    T: Indexer<Label = IdxVec3d>,
{
    type Output = f64;

    #[inline]
    fn index(&self, index: T) -> &Self::Output {
        match index.idx_label() {
            IdxVec3d::Fst => &self.0,
            IdxVec3d::Snd => &self.1,
            IdxVec3d::Trd => &self.2,
        }
    }
}

impl From<f64> for Vec3d {
    fn from(value: f64) -> Self {
        Vec3d(value, value, value)
    }
}

impl PartialEq for Vec3d {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl PartialEq<f64> for Vec3d {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other && self.1 == *other && self.2 == *other
    }
}

impl Neg for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3d(-self[0], -self[1], -self[2])
    }
}

impl Add for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3d(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Add<f64> for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Vec3d(self[0] + rhs, self[1] + rhs, self[2] + rhs)
    }
}

impl Add<Vec3d> for f64 {
    type Output = Vec3d;

    #[inline]
    fn add(self, rhs: Vec3d) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for Vec3d {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign<f64> for Vec3d {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Sub<f64> for Vec3d {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3d(self[0] - rhs, self[1] - rhs, self[2] - rhs)
    }
}

impl Sub<Vec3d> for f64 {
    type Output = Vec3d;

    #[inline]
    fn sub(self, rhs: Vec3d) -> Self::Output {
        Vec3d(self - rhs[0], self - rhs[1], self - rhs[2])
    }
}

impl SubAssign for Vec3d {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl SubAssign<f64> for Vec3d {
    #[inline]
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}

impl Mul for Vec3d {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3d(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl Mul<f64> for Vec3d {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3d(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl Mul<Vec3d> for f64 {
    type Output = Vec3d;

    #[inline]
    fn mul(self, rhs: Vec3d) -> Self::Output {
        rhs * self
    }
}

impl MulAssign for Vec3d {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl MulAssign<f64> for Vec3d {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3d {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3d(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl Div for Vec3d {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Vec3d(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl DivAssign for Vec3d {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl DivAssign<f64> for Vec3d {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

///---------------------------------------------------------------
/// # methods of Vec2d
///
impl Vec2d {}

impl Add for Vec2d {
    type Output = Vec2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec2d(self[0] + rhs[0], self[1] + rhs[1])
    }
}

impl ops::Index<usize> for Vec2d {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 2);
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("{ERROR}: index out of range"),
        }
    }
}

impl<T> ops::Index<T> for Vec2d
where
    T: Indexer<Label = IdxVec2d>,
{
    type Output = f64;

    #[inline]
    fn index(&self, index: T) -> &Self::Output {
        match index.idx_label() {
            IdxVec2d::Fst => &self.0,
            IdxVec2d::Snd => &self.1,
        }
    }
}
