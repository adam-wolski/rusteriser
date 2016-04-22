use std::path;
use tobj;
use cgmath::*;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub uv: Vector2<f32>,
}


#[derive(Debug, Clone)]
pub struct Face {
    pub verts: Vec<Vertex>,
}


#[derive(Debug, Clone)]
pub struct Model {
    pub faces: Vec<Face>,
}

impl Model {
    // TODO: Error checks when loading model and return Result type.
    pub fn load(path: &path::Path) -> Model {
        let (models, _) = tobj::load_obj(path).unwrap();
        // TODO: We assume it's just one model for now.
        let mesh: &tobj::Mesh = &models[0].mesh;
        Model { faces: Model::create_faces(&mesh) }
    }

    fn create_faces(mesh: &tobj::Mesh) -> Vec<Face> {
        let mut faces: Vec<Face> = Vec::with_capacity(mesh.indices.len() / 3);
        for i in (0..mesh.indices.len()).filter(|i| i % 3 == 0) {

            let mut face = Face { verts: Vec::with_capacity(3) };
            // TODO: Error check for when there are no normals or uvs provided.

            for m in 0..3 {
                face.verts.push(Vertex {
                    pos: Vector3::<f32>::new(mesh.positions[mesh.indices[i + m] as usize * 3],
                                             mesh.positions[mesh.indices[i + m] as usize * 3 + 1],
                                             mesh.positions[mesh.indices[i + m] as usize * 3 + 2]),
                    normal: if mesh.normals.is_empty() {
                        Vector3::<f32>::new(1.0, 1.0, 1.0)
                    } else {
                        Vector3::<f32>::new(mesh.normals[mesh.indices[i + m] as usize * 3],
                                            mesh.normals[mesh.indices[i + m] as usize * 3 + 1],
                                            mesh.normals[mesh.indices[i + m] as usize * 3 + 2])
                    },
                    uv: if mesh.texcoords.is_empty() {
                        Vector2::<f32>::new(0.0, 0.0)
                    } else {
                        Vector2::<f32>::new(mesh.texcoords[mesh.indices[i + m] as usize * 2],
                                            mesh.texcoords[mesh.indices[i + m] as usize * 2 + 1])
                    },
                });
            }
            faces.push(face);
        }
        faces
    }
}
