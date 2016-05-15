use std::path;
use tobj;
use cgmath::*;


#[derive(Debug)]
pub enum ModelError {
    CouldNotLoadFile,
    NoTexCoords,
    NoNormals,
}


#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
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
    pub fn load(path: &path::Path) -> Result<Model, ModelError> {
        let (models, _) = match tobj::load_obj(path) {
            Ok(model_and_mats) => model_and_mats,
            Err(e) => {
                error!("{:?}", e);
                return Err(ModelError::CouldNotLoadFile);
            }
        };
        // TODO: We assume it's just one model for now.
        let mesh: &tobj::Mesh = &models[0].mesh;
        Ok(Model { faces: try!(Model::create_faces(&mesh)) })
    }

    fn create_faces(mesh: &tobj::Mesh) -> Result<Vec<Face>, ModelError> {
        if mesh.normals.is_empty() {
            return Err(ModelError::NoNormals);
        }
        if mesh.texcoords.is_empty() {
            return Err(ModelError::NoTexCoords);
        }
        let mut faces: Vec<Face> = Vec::with_capacity(mesh.indices.len() / 3);
        for i in (0..mesh.indices.len()).filter(|i| i % 3 == 0) {

            let mut face = Face { verts: Vec::with_capacity(3) };

            for m in 0..3 {
                face.verts.push(Vertex {
                    pos: Vector3::<f32>::new(mesh.positions[mesh.indices[i + m] as usize * 3],
                                             mesh.positions[mesh.indices[i + m] as usize * 3 + 1],
                                             mesh.positions[mesh.indices[i + m] as usize * 3 + 2]),
                    normal: Vector3::<f32>::new(mesh.normals[mesh.indices[i + m] as usize * 3],
                                                mesh.normals[mesh.indices[i + m] as usize * 3 + 1],
                                                mesh.normals[mesh.indices[i + m] as usize * 3 + 2]),
                    texcoord: Vector2::<f32>::new(mesh.texcoords[mesh.indices[i + m] as usize * 2],
                                                  mesh.texcoords[mesh.indices[i + m] as usize * 2 +
                                                                 1]),
                });
            }
            faces.push(face);
        }
        Ok(faces)
    }
}
