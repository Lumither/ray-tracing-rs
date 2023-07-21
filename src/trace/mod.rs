// this is the Surface, which stores all the surfaces, including trig, trig_patch, and sphere

use std::{cell::RefCell, rc::Rc};

pub use surface::{Surface, Triangle};

use crate::{trace::nnf::read_nnf, vectors::Vec3d};

use self::basic::{Fill, Light};

pub mod basic;
pub mod nnf;
mod surface;

pub type SurfPtr = Rc<RefCell<dyn Surface>>;

type Color = Vec3d;

#[derive(Debug)]
pub struct Config {
    fname: Vec<String>,
    aperture: f64,
    max_depth: u8,
    samples: u32,
    /// whether the objects is colored or not,
    color: bool,
}

pub struct Tracer {
    /// background color
    pub bcolor: Color,
    /// position of your eye
    pub eye: Vec3d,
    /// surfaces that we encounter
    pub surfaces: Vec<(Box<dyn Surface>, Fill)>,
    /// light sources
    pub lights: Vec<Light>,
    pub to_file: Box<str>,
}

impl Config {
    pub fn load() -> Config {
        let mut config = Config {
            fname: Vec::with_capacity(2),
            color: false,
            aperture: 0.0,
            max_depth: 5,
            samples: 1,
        };
        let mut args = std::env::args().skip(1);

        if args.len() == 0 {
            return config;
        }

        while let Some(arg) = args.next() {
            match &arg[..] {
                "-a" => {
                    config.aperture = args
                        .next()
                        .expect("Should have a aperture value")
                        .parse()
                        .expect("aperture value should be a number!")
                }
                "-s" => {
                    config.samples = args
                        .next()
                        .expect("Should have a sampling value")
                        .parse()
                        .expect("sampling value should be a integer!")
                }
                "-c" => config.color = true,
                "-d" => {
                    config.max_depth = args
                        .next()
                        .expect("Should have a max ray depth")
                        .parse()
                        .expect("max ray depth value should be a number!")
                }
                fname if config.fname.len() < 2 => config.fname.push(fname.into()),
                _ => (),
            }
        }
        if config.fname.len() != 2 {
            println!("usage: trace input.nff output.ppm [opts]");
            std::process::exit(0);
        }
        config
        // todo!("method not implemented!")
    }
}

impl Tracer {
    #[inline]
    pub fn new(config: Config) -> Tracer {
        Self::from_config(config)
    }

    fn from_config(config: Config) -> Tracer {
        let nnf_file = read_nnf(&config.fname[0][..]);
        todo!("method not implemented")
    }
}
