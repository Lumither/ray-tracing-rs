//! ray

use crate::vectors::Vec3d;

use super::Color;

pub struct Ray {
    pub eye: Vec3d,
    pub dir: Vec3d,
    pub depth: u8,
}

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
    pub color: Color
}