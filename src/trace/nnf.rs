use std::fs;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use crate::trace::basic::{Camara, Light};
use crate::trace::{Color, Surface};
use crate::vectors::{Vec2d, Vec3d};
use crate::ERROR;

use super::basic::Fill;
use super::surface::Sphere;

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

    let mut background_color: Color = Color::new();
    let mut camara: Option<Camara> = None;
    let mut lights: Vec<Light> = Vec::new();
    let mut surfaces: Vec<Box<dyn Surface>> = Vec::new();
    let mut fill = Rc::new(Fill::new());

    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        match line.chars().next() {
            Some('b') => {
                let mut ss = line.split_whitespace().skip(1);
                // ss.next();
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
                camara = Some(Camara {
                    from: tmp_from,
                    at: tmp_at,
                    up: tmp_up,
                    angle: tmp_angle,
                    hither: tmp_hither,
                    resolution: tmp_resolution,
                })
            }
            Some('p') => {
                let mut ss = line.split_whitespace();
                let patch = match ss.next() {
                    Some("pp") => true,
                    Some("p") => false,
                    x => panic!("{} no such type of polygon header: {}", ERROR, x.unwrap()),
                };
            }
            Some('s') => {
                let mut ss = line.split_whitespace().skip(1);
                if let (Some(x), Some(y), Some(z), Some(r)) =
                    (ss.next(), ss.next(), ss.next(), ss.next())
                {
                    surfaces.push(Box::new(Sphere {
                        center: Vec3d(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()),
                        material: Rc::clone(&fill),
                        radius: r.parse().unwrap(),
                    }));
                }
            }
            Some('f') => {
                let mut ss = line.split_whitespace().skip(1);
                if let (
                    Some(r),
                    Some(g),
                    Some(b),
                    Some(kd),
                    Some(ks),
                    Some(shine),
                    Some(t),
                    Some(ior),
                ) = (
                    ss.next(),
                    ss.next(),
                    ss.next(),
                    ss.next(),
                    ss.next(),
                    ss.next(),
                    ss.next(),
                    ss.next(),
                ) {
                    fill = Rc::new(Fill {
                        c: Vec3d(r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap()),
                        kd: kd.parse().unwrap(),
                        ks: ks.parse().unwrap(),
                        shine: shine.parse().unwrap(),
                        t: t.parse().unwrap(),
                        ior: ior.parse().unwrap(),
                    })
                }
            }
            Some('l') => {
                let mut ss = line.split_whitespace().skip(1);
                let mut tmp_pos: Vec3d = Vec3d(0.0, 0.0, 0.0);
                let mut tmp_color: Color = Vec3d(255.0, 255.0, 255.0);
                // ss.next();
                if let (Some(x), Some(y), Some(z)) = (ss.next(), ss.next(), ss.next()) {
                    tmp_pos = Vec3d(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
                }
                if let (Some(r), Some(g), Some(b)) = (ss.next(), ss.next(), ss.next()) {
                    tmp_color = Vec3d(r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap());
                }
                lights.push(Light {
                    position: tmp_pos,
                    color: tmp_color,
                });
            }
            _ => {}
        }
    }

    Ok(NnfFile {
        background_color,
        camara: camara.expect("the nff file should include information of camera"),
        lights,
        surfaces,
    })

    // todo!("to be finished");
}
