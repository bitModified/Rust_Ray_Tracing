use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::BuildHasherDefault,
};

use crate::{
    math::tuple::Tuple,
    shape::{triangle::Triangle, Object, Shape},
};

pub struct WavefrontObj {
    groups: HashMap<String, Vec<Triangle>, BuildHasherDefault<DefaultHasher>>,
    #[cfg(test)]
    vertices: Vec<Tuple>,
    #[cfg(test)]
    normals: Vec<Tuple>,
}

impl WavefrontObj {
    pub fn to_group(self) -> Object {
        Object::group(
            self.groups
                .into_iter()
                .map(|(_, triangles)| {
                    let triangles = triangles
                        .into_iter()
                        .map(|triangle| Object::new(Shape::Triangle(triangle)))
                        .collect();

                    Object::group(triangles)
                })
                .collect(),
        )
    }

    pub fn from_file(file_path: &str) -> std::io::Result<Object> {
        let file_contents = std::fs::read_to_string(file_path)?;
        let obj = WavefrontObj::from_file_contents(&file_contents)?;
        Ok(obj.to_group())
    }

    pub fn from_file_contents(file_contents: &str) -> std::io::Result<WavefrontObj> {
        let mut vertices = vec![];
        let mut normals = vec![];

        let mut current_group = "default";

        let map_hasher = BuildHasherDefault::<DefaultHasher>::default();
        let mut groups: HashMap<String, Vec<Triangle>, _> = HashMap::with_hasher(map_hasher);

        for line in file_contents.lines() {
            if let Some((node_type, rest)) = line.split_once(" ") {
                match node_type {
                    "v" => {
                        let mut rest = rest.split_ascii_whitespace();
                        let x = rest.next().unwrap().parse::<f64>().unwrap();
                        let y = rest.next().unwrap().parse::<f64>().unwrap();
                        let z = rest.next().unwrap().parse::<f64>().unwrap();

                        vertices.push(Tuple::point(x, y, z));
                    }
                    "vn" => {
                        let mut rest = rest.split_ascii_whitespace();
                        let x = rest.next().unwrap().parse::<f64>().unwrap();
                        let y = rest.next().unwrap().parse::<f64>().unwrap();
                        let z = rest.next().unwrap().parse::<f64>().unwrap();

                        normals.push(Tuple::vector(x, y, z));
                    }
                    "f" => {
                        let rest = rest.split_ascii_whitespace();
                        let mut indices = rest.map(|attr| {
                            let mut it = attr.split('/').map(|i| i.parse::<usize>().ok());

                            let vertex = it.next().unwrap().unwrap() - 1;
                            let texture = it.next().flatten().map(|t| t - 1);
                            let normal = it.next().flatten().map(|t| t - 1);

                            (vertex, texture, normal)
                        });

                        let (start_index, _, normal1) = indices.next().unwrap();
                        for window in indices.collect::<Vec<_>>().windows(2) {
                            if let [(index2, _, normal2), (index3, _, normal3)] = window {
                                let entry = groups.entry(current_group.to_owned());
                                let triangle = match (normal1, normal2, normal3) {
                                    (Some(n1), Some(n2), Some(n3)) => Triangle::smooth(
                                        vertices[start_index],
                                        vertices[*index2],
                                        vertices[*index3],
                                        normals[n1],
                                        normals[*n2],
                                        normals[*n3],
                                    ),
                                    _ => Triangle::new(
                                        vertices[start_index],
                                        vertices[*index2],
                                        vertices[*index3],
                                    ),
                                };

                                entry.or_insert(vec![]).push(triangle);
                            }
                        }
                    }
                    "g" => {
                        current_group = rest;
                    }
                    _ => {}
                }
            }
        }

        Ok(WavefrontObj {
            groups,
            #[cfg(test)]
            vertices,
            #[cfg(test)]
            normals,
        })
    }
}
