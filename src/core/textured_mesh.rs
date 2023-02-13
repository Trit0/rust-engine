use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::ptr::null;
use obj::{load_obj, Obj};
use crate::core::component::Component;
use crate::core::mesh::{Mesh, Normal, Vertex};
use crate::math::vector_3f::Vector3F;
use crate::utils::string_util;
use crate::utils::string_util::StringUtils;

#[derive(Copy, Clone, Debug)]
pub struct TexCoords {
    tex_coords: (f32, f32)
}

#[derive(Debug)]
pub struct TexturedMesh {
    pub mesh: Mesh,
    pub texture_coords: Vec<TexCoords>,
}

impl TexturedMesh {
    pub fn load(model: &str) -> Self {
        let buffer = BufReader::new(File::open(model).unwrap());
        let lines = buffer.lines();

        let mut li: String;
        let mut lv1: String;
        let mut lv2: String;
        let mut lv3: String;
        let mut v: String;
        let mut t: String;
        let mut n: String;
        let mut temp: String;

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut tex_coords: Vec<TexCoords> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let mut max_x: f32 = 0.0;
        let mut max_y: f32 = 0.0;
        let mut max_z: f32 = 0.0;
        let mut min_x: f32 = 0.0;
        let mut min_y: f32 = 0.0;
        let mut min_z: f32 = 0.0;

        let mut v_index: u16;
        let mut t_index: u16;
        let mut n_index: u16;

        let mut face_count: u32 = 0;


        for line_r in lines {
            let line = match line_r {
                Ok(line) => line,
                Err(_err) => return TexturedMesh {
                    texture_coords: Vec::new(),
                    mesh: Mesh::new()
                },
            };
            li = line.substring(0, Some(line.find(" ").unwrap())).to_string();

            match li.as_str() {
                "v" => {
                    lv1 = line.substring(line.find(" ").unwrap() + 1, None).to_string();
                    lv2 = lv1.substring(lv1.find(" ").unwrap() + 1, None).to_string();
                    lv3 = lv2.substring(lv2.find(" ").unwrap() + 1, None).to_string();
                    lv1 = lv1.substring(0, Some(lv1.find(" ").unwrap())).to_string();
                    lv2 = lv2.substring(0, Some(lv2.find(" ").unwrap())).to_string();

                    let x: f32 = lv1.parse::<f32>().unwrap();
                    let y: f32 = lv2.parse::<f32>().unwrap();
                    let z: f32 = lv3.parse::<f32>().unwrap();

                    vertices.push(Vertex { position: (x, y, z) });

                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                    if z > max_z {
                        max_z = z;
                    }
                    if x < min_x {
                        min_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if z < min_z {
                        min_z = z;
                    }
                }
                "vt" => {
                    lv1 = line.substring(line.find(" ").unwrap() + 1, None).to_string();
                    lv2 = lv1.substring(lv1.find(" ").unwrap() + 1, None).to_string();
                    lv1 = lv1.substring(0, Some(lv1.find(" ").unwrap())).to_string();
                    tex_coords.push( TexCoords { tex_coords: (lv1.parse::<f32>().unwrap(), lv2.parse::<f32>().unwrap()) } );
                }
                "vn" => {
                    lv1 = line.substring(line.find(" ").unwrap() + 1, None).to_string();
                    lv2 = lv1.substring(lv1.find(" ").unwrap() + 1, None).to_string();
                    lv3 = lv2.substring(lv2.find(" ").unwrap() + 1, None).to_string();
                    lv1 = lv1.substring(0, Some(lv1.find(" ").unwrap())).to_string();
                    lv2 = lv2.substring(0, Some(lv2.find(" ").unwrap())).to_string();
                    normals.push( Normal { normal: (lv1.parse::<f32>().unwrap(), lv2.parse::<f32>().unwrap(), lv3.parse::<f32>().unwrap()) } );
                }
                "f" => {
                    lv1 = line.substring(line.find(" ").unwrap() + 1, None).to_string();
                    lv2 = lv1.substring(lv1.find(" ").unwrap() + 1, None).to_string();
                    lv3 = lv2.substring(lv2.find(" ").unwrap() + 1, None).to_string();
                    lv1 = lv1.substring(0, Some(lv1.find(" ").unwrap())).to_string();
                    lv2 = lv2.substring(0, Some(lv2.find(" ").unwrap())).to_string();

                    for lv in [lv1, lv2, lv3] {
                        v = lv.substring(0, Some(lv.find("/").unwrap())).to_string();
                        temp = lv.substring(lv.find("/").unwrap() + 1, None).to_string();
                        t = temp.substring(0, Some(temp.find("/").unwrap())).to_string();
                        n = temp.substring(temp.find("/").unwrap() + 1, None).to_string();

                        v_index = v.parse::<u16>().unwrap() - 1;
                        t_index = t.parse::<u16>().unwrap() - 1;
                        n_index = n.parse::<u16>().unwrap() - 1;

                        indices.push(v_index);
                        indices.push(t_index);
                        indices.push(n_index);
                    }
                    face_count += 1;
                }
                _ => {}
            }
        }

        let mesh = Mesh {
            vertices: vertices.clone(),
            normals: normals.clone(),
            indices: indices.clone(),
            face_count,
            size: Vector3F::new((max_x - min_x) as f64, (max_y - min_y) as f64, (max_z - min_z) as f64)
        };

        return TexturedMesh {
            mesh,
            texture_coords: tex_coords.clone()
        };
    }
}

impl Component for TexturedMesh {

}
