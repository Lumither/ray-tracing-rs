use std::fs;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use crate::trace::basic::{Camera, Light};
use crate::trace::surface::TrianglePatch;
use crate::trace::{Color, Surface, Triangle};
use crate::vectors::{Vec2d, Vec3d};
use crate::ERROR;

use super::basic::Fill;
use super::surface::Sphere;

#[derive(Debug)]
pub enum NnfCreateError {
    ReadingError,
    CreateError,
    ValidityError,
}

type MaterialSurf = (Box<dyn Surface>, Rc<Fill>);

pub struct NnfFile {
    pub background_color: Color,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub surfaces: Vec<MaterialSurf>,
}

pub fn read_nnf(fname: &str) -> Result<NnfFile, NnfCreateError> {
    let file = match fs::File::open(fname) {
        Ok(file) => file,
        Err(_) => return Err(NnfCreateError::ReadingError),
    };

    let reader = BufReader::new(file);
    let mut background_color: Color = Color::new();
    let mut camera: Option<Camera> = None;
    let mut lights: Vec<Light> = Vec::new();
    let mut surfaces: Vec<MaterialSurf> = Vec::new();
    let mut fill = Rc::new(Fill::new());

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
                camera = Some(Camera {
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
                    x => panic!("{ERROR} no such type of polygon header: {}", x.unwrap()),
                };
                let mut vertices: Vec<Vec3d> = Vec::new();
                let mut normals: Vec<Vec3d> = Vec::new();
                let mut ss = line.split_whitespace().skip(1);
                let num: u128 = ss.next().unwrap().parse().unwrap();
                for _ in 0..num {
                    if let Some(Ok(_line)) = lines.next() {
                        let mut tmp_vertice = Vec3d(0.0, 0.0, 0.0);
                        let mut tmp_normal = Vec3d(0.0, 0.0, 0.0);
                        let mut iter_line = _line.split_whitespace();
                        if patch {
                            if let (Some(v1), Some(v2), Some(v3), Some(n1), Some(n2), Some(n3)) = (
                                iter_line.next(),
                                iter_line.next(),
                                iter_line.next(),
                                iter_line.next(),
                                iter_line.next(),
                                iter_line.next(),
                            ) {
                                tmp_vertice = Vec3d(
                                    v1.parse().unwrap(),
                                    v2.parse().unwrap(),
                                    v3.parse().unwrap(),
                                );
                                tmp_normal = Vec3d(
                                    n1.parse().unwrap(),
                                    n2.parse().unwrap(),
                                    n3.parse().unwrap(),
                                );
                            }
                        } else if let (Some(v1), Some(v2), Some(v3)) =
                            (iter_line.next(), iter_line.next(), iter_line.next())
                        {
                            tmp_vertice = Vec3d(
                                v1.parse().unwrap(),
                                v2.parse().unwrap(),
                                v3.parse().unwrap(),
                            );
                        }
                        vertices.push(tmp_vertice);
                        normals.push(tmp_normal);
                    }
                }
                let mut make_triangles = false;
                if vertices.len() == 3 {
                    if patch {
                        surfaces.push((
                            Box::new(TrianglePatch {
                                _super: Triangle {
                                    a: vertices[0],
                                    b: vertices[1],
                                    c: vertices[2],
                                },
                                n1: normals[0],
                                n2: normals[1],
                                n3: normals[2],
                            }),
                            Rc::clone(&fill),
                        ));
                    } else {
                        surfaces.push((
                            Box::new(Triangle {
                                a: vertices[0],
                                b: vertices[1],
                                c: vertices[2],
                            }),
                            Rc::clone(&fill),
                        ));
                    }
                } else if vertices.len() == 4 {
                    let n0: Vec3d = (vertices[1] - vertices[0]).cross(&(vertices[2] - vertices[0]));
                    let n1: Vec3d = (vertices[2] - vertices[1]).cross(&(vertices[3] - vertices[1]));
                    let n2: Vec3d = (vertices[3] - vertices[2]).cross(&(vertices[0] - vertices[2]));
                    let n3: Vec3d = (vertices[0] - vertices[3]).cross(&(vertices[1] - vertices[3]));
                    if n0.dot(&n1) > 0.0 && n0.dot(&n2) > 0.0 && n0.dot(&n3) > 0.0 {
                        make_triangles = true;
                        if patch {
                            surfaces.push((
                                Box::new(TrianglePatch {
                                    _super: Triangle {
                                        a: vertices[0],
                                        b: vertices[1],
                                        c: vertices[2],
                                    },
                                    n1: normals[0],
                                    n2: normals[1],
                                    n3: normals[2],
                                }),
                                Rc::clone(&fill),
                            ));
                            surfaces.push((
                                Box::new(TrianglePatch {
                                    _super: Triangle {
                                        a: vertices[0],
                                        b: vertices[2],
                                        c: vertices[3],
                                    },
                                    n1: normals[0],
                                    n2: normals[2],
                                    n3: normals[3],
                                }),
                                Rc::clone(&fill),
                            ));
                        } else {
                            surfaces.push((
                                Box::new(Triangle {
                                    a: vertices[0],
                                    b: vertices[1],
                                    c: vertices[2],
                                }),
                                Rc::clone(&fill),
                            ));
                            surfaces.push((
                                Box::new(Triangle {
                                    a: vertices[0],
                                    b: vertices[2],
                                    c: vertices[3],
                                }),
                                Rc::clone(&fill),
                            ));
                        }
                    }
                    if !make_triangles {
                        eprintln!("I didn't make triangles.  Poly not flat or more than quad.");
                    }
                }
            }
            Some('s') => {
                let mut ss = line.split_whitespace().skip(1);
                if let (Some(x), Some(y), Some(z), Some(r)) =
                    (ss.next(), ss.next(), ss.next(), ss.next())
                {
                    surfaces.push((
                        Box::new(Sphere {
                            center: Vec3d(
                                x.parse().unwrap(),
                                y.parse().unwrap(),
                                z.parse().unwrap(),
                            ),
                            radius: r.parse().unwrap(),
                        }),
                        Rc::clone(&fill),
                    ));
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
        camera: camera.expect("the nff file should include information of camera"),
        lights,
        surfaces,
    })

    // todo!("to be finished");
}
