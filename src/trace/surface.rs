// triangle surface

use crate::trace::basic::Fill;
use crate::vectors::Vec3d;

use super::basic::{HitRecord, Ray};

pub struct Sphere {
    pub material: Fill,
    pub center: Vec3d,
    pub radius: f64,
}

pub struct Triangle {
    pub material: Fill,
    pub a: Vec3d,
    pub b: Vec3d,
    pub c: Vec3d,
}

pub struct TrianglePatch {
    pub material: Fill,
    pub _super: Triangle,
    pub n1: Vec3d,
    pub n2: Vec3d,
    pub n3: Vec3d,
}

pub trait Surface {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool;
}

impl Surface for Sphere {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool {
        todo!("The method has not been implemented!")
    }
}

impl Surface for Triangle {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool {
        let is_hit: bool = false;
        todo!("The method has not yet been implemented!")
    }
}

impl Surface for TrianglePatch {
    fn intersect(&mut self, ray: &Ray, t0: f64, t1: f64, hits: &mut HitRecord) -> bool {
        let is_hit: bool = false;
        todo!("The method has not yet been implemented!")
    }
}
