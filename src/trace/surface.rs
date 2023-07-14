// triangle surface

use crate::vectors::Vec3d;

use super::{
    basic::{HitRecord, Ray},
    Color,
};

pub trait Surface {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> Color;
}

pub struct Triangle {}

impl Surface for Triangle {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> Color {
        Vec3d(0.0, 0.0, 0.0)
    }
}

pub struct Sphere {}
impl Surface for Sphere {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> Color {
        Vec3d(0.0, 0.0, 0.0)
    }
}
