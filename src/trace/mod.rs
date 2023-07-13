// this is the Surface, which stores all the surfaces, including trig, trig_patch, and sphere

pub mod nnf_reader;
pub mod basic;
mod surface;

use crate::vectors::Vec3d;

pub use surface::{Surface, Triangle};

use self::basic::Fill;

type Color = Vec3d;

struct Tracer {
    surfaces: Vec<(Box<dyn Surface>, Fill)>
}