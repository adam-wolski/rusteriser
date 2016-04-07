use std::path;
use tobj;
use cgmath::Vector3;

pub struct Model {
    pub faces: Vec<Vec<Vector3<f32>>>,
}

impl Model {
    // TODO: Error checks when loading model and return Result type.
    pub fn load(path: &path::Path) -> Model {
        let (models, _) = tobj::load_obj(path).unwrap();
        // TODO: We assume it's just one model for now.
        let mesh: &tobj::Mesh = &models[0].mesh;
        Model { faces: Model::create_faces(&mesh) }
    }

    fn create_faces(mesh: &tobj::Mesh) -> Vec<Vec<Vector3<f32>>> {
        let mut faces: Vec<Vec<Vector3<f32>>> = Vec::with_capacity(mesh.indices.len() / 3);
        for i in (0..mesh.indices.len()).filter(|i| i % 3 == 0) {
            let mut face: Vec<Vector3<f32>> = Vec::with_capacity(3);
            face.push(Vector3::<f32>::new(mesh.positions[mesh.indices[i] as usize * 3],
                                          mesh.positions[mesh.indices[i] as usize * 3 + 1],
                                          mesh.positions[mesh.indices[i] as usize * 3 + 2]));
            face.push(Vector3::<f32>::new(mesh.positions[mesh.indices[i + 1] as usize * 3],
                                          mesh.positions[mesh.indices[i + 1] as usize * 3 + 1],
                                          mesh.positions[mesh.indices[i + 1] as usize * 3 + 2]));
            face.push(Vector3::<f32>::new(mesh.positions[mesh.indices[i + 2] as usize * 3],
                                          mesh.positions[mesh.indices[i + 2] as usize * 3 + 1],
                                          mesh.positions[mesh.indices[i + 2] as usize * 3 + 2]));
            faces.push(face);
        }
        faces
    }
}
