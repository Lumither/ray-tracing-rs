// this is the Surface, which stores all the surfaces, including trig, trig_patch, and sphere

pub mod basic;
pub mod nnf_reader;
mod surface;

use std::{cell::RefCell, rc::Rc};

use crate::vectors::Vec3d;

pub use surface::{Surface, Triangle};

use self::basic::Fill;

pub type SurfPtr = Rc<RefCell<dyn Surface>>;

type Color = Vec3d;

struct Tracer {
    surfaces: Vec<(SurfPtr, Fill)>,
}
