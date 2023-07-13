// triangle surface

use crate::vectors::Vec3d;

use super::{basic::Ray, Color};

pub trait Surface {
    fn intersect(&mut self, ray: Ray, t0: f64, t1: f64) -> Color;
}

pub struct Triangle {}

impl Surface for Triangle {
    fn intersect(&mut self, ray: Ray, t0: f64, t1: f64) -> Color {
        Vec3d(0.0, 0.0, 0.0)
    }
}
