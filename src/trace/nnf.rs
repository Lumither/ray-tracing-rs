use std::fs;
use std::io::{BufRead, BufReader};

use crate::trace::basic::{Camara, Light};
use crate::trace::{Color, Surface};
use crate::vectors::{Vec2d, Vec3d};

#[derive(Debug)]
pub struct NnfReadError;

pub struct NnfFile {
    pub background_color: Color,
    pub camara: Camara,
    pub lights: Vec<Light>,
    pub surfaces: Vec<Box<dyn Surface>>,
}

pub fn read_nnf(fname: &str) -> Result<NnfFile, NnfReadError> {
    let file = fs::File::open(fname).expect("file DNE");
    let reader = BufReader::new(file);

    let mut background_color: Color;
    let mut camara: Camara;
    let mut lights: Vec<Light> = Vec::new();
    let mut surfaces: Vec<Box<dyn Surface>> = Vec::new();

    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        match line.chars().next() {
            Some('b') => {
                let mut ss = line.split_whitespace();
                ss.next();
                if let (Some(r), Some(g), Some(b)) = (ss.next(), ss.next(), ss.next()) {
                    background_color = Color {
                        0: r.parse().unwrap(),
                        1: g.parse().unwrap(),
                        2: b.parse().unwrap(),
                    }
                }
            }
            Some('v') => {
                let mut tmp_from: Vec3d = Vec3d(0.0, 0.0, 0.0);
                let mut tmp_at: Vec3d = Vec3d(0.0, 0.0, 0.0);
                let mut tmp_up: Vec3d = Vec3d(0.0, 0.0, 0.0);
                let mut tmp_angle: f64 = 0.0;
                let mut tmp_hither: f64 = 0.0;
                let mut tmp_resolution: Vec2d = Vec2d(0.0, 0.0);
                for _ in 0..5 {
                    if let Some(Ok(_next_line)) = lines.next() {
                        let mut ss = line.split_whitespace();
                        match ss.next() {
                            Some("from") => {
                                if let (Some(x), Some(y), Some(z)) =
                                    (ss.next(), ss.next(), ss.next())
                                {
                                    tmp_from = Vec3d(
                                        x.parse().unwrap(),
                                        y.parse().unwrap(),
                                        z.parse().unwrap(),
                                    )
                                }
                            }
                            Some("at") => {
                                if let (Some(x), Some(y), Some(z)) =
                                    (ss.next(), ss.next(), ss.next())
                                {
                                    tmp_at = Vec3d(
                                        x.parse().unwrap(),
                                        y.parse().unwrap(),
                                        z.parse().unwrap(),
                                    )
                                }
                            }
                            Some("up") => {
                                if let (Some(x), Some(y), Some(z)) =
                                    (ss.next(), ss.next(), ss.next())
                                {
                                    tmp_up = Vec3d(
                                        x.parse().unwrap(),
                                        y.parse().unwrap(),
                                        z.parse().unwrap(),
                                    )
                                }
                            }
                            Some("angle") => {
                                if let Some(angle) = ss.next() {
                                    tmp_angle = angle.parse().unwrap();
                                }
                            }
                            Some("hither") => {
                                if let Some(hither) = ss.next() {
                                    tmp_hither = hither.parse().unwrap();
                                }
                            }
                            Some("resolution") => {
                                if let (Some(x), Some(y)) = (ss.next(), ss.next()) {
                                    tmp_resolution = Vec2d(x.parse().unwrap(), y.parse().unwrap());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                camara = Camara {
                    from: tmp_from,
                    at: tmp_at,
                    up: tmp_up,
                    angle: tmp_angle,
                    hither: tmp_hither,
                    resolution: tmp_resolution,
                }
            }
            Some('p') => {}
            Some('s') => {}
            Some('f') => {}
            Some('l') => {
                let mut ss = line.split_whitespace();
                let mut tmp_pos: Vec3d = Vec3d(0.0, 0.0, 0.0);
                let mut tmp_color: Color = Vec3d(255.0, 255.0, 255.0);
                ss.next();
                if let (Some(x), Some(y), Some(z)) = (ss.next(), ss.next(), ss.next()) {
                    tmp_pos = Vec3d(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
                }
                if ss.clone().count() > 0 {
                    if let (Some(r), Some(g), Some(b)) = (ss.next(), ss.next(), ss.next()) {
                        tmp_color =
                            Vec3d(r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap());
                    }
                }
                lights.push(Light {
                    position: tmp_pos,
                    color: tmp_color,
                });
            }
            _ => {}
        }
    }

    todo!("to be finished");
}
