// triangle surface

use crate::vectors::Vec3d;

use super::{
    basic::{HitRecord, Ray},
    Color,
};

pub trait Surface {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool;
}

pub struct Triangle {}

impl Surface for Triangle {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool {
        let is_hit: bool = false;
        todo!("The method has not been implemented!")
    }
}

pub struct Sphere {}
impl Surface for Sphere {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool {
        todo!("The method has not been implemented!")
    }
}
