//! ray

use crate::vectors::{Vec2d, Vec3d};

use super::Color;

pub struct Ray {
    pub cam_pos: Vec3d,
    // camera position
    pub dir: Vec3d,
    pub depth: u8,
}

#[derive(Default)]
pub struct Fill {
    pub c: Color,
    pub kd: f64,
    pub ks: f64,
    pub shine: f64,
    pub t: f64,
    pub ior: f64,
}

pub struct HitRecord {
    pub t: f64,
    pub hit_point: Vec3d,
    pub normal: Vec3d,
    pub view: Vec3d,
}

pub struct Light {
    pub position: Vec3d,
    pub color: Color,
}

pub struct Camera {
    pub from: Vec3d,
    // camera position
    pub at: Vec3d,
    // position to be at the center of img
    pub up: Vec3d,
    // a vector defining which direction is up, as an XYZ vector
    pub angle: f64,
    // pov
    pub hither: f64,
    // distance of the hither plane (if any) from the camera
    pub resolution: Vec2d, // resolution of camera
}

pub struct BGColor {
    pub color: Color,
}

impl Fill {
    #[inline]
    pub fn new() -> Fill {
        Fill::default()
    }
}
