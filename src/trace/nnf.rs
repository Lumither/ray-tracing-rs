use std::fs;
use crate::trace::basic::{BGColor, Camara, Light};
use crate::trace::Surface;

struct NnfReadError;

struct NnfFile {
    pub background_color: BGColor,
    pub camara: Camara,
    pub lights: Box<Light>,
    pub surfaces: Box<dyn Surface>,
}

fn read_nnf(fname: &str) -> Result<NnfFile, NnfReadError> {
    let file = fs::read_to_string(fname).expect("file DNE");
    println!("{file}");
    todo!("method not implemented")
}
