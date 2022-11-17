use crate::department::preview::position::Pos3;
use crate::department::preview::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct RenderObject {
    pub vertexes: Vec<Pos3>,
    pub normals: Vec<Pos3>,
    pub texcoords: Vec<Pos3>,
    pub indexes: Vec<usize>,
    pub normal_indexes: Vec<usize>,
    pub tex_indexes: Vec<usize>,
}

impl RenderObject {
    pub fn new() -> Self {
        Self {
            vertexes: Vec::new(),
            normals: Vec::new(),
            texcoords: Vec::new(),
            indexes: Vec::new(),
            normal_indexes: Vec::new(),
            tex_indexes: Vec::new(),
        }
    }

    pub fn mul_matrix(&mut self, mat: &Matrix<3,3>) {
        for i in self.vertexes.iter_mut() {
            let r3:& Pos3 = i;
            *i = r3 * mat;
        }
    }

    pub fn from_vec(vertexes: Vec<Pos3>, indexes: Vec<usize>) -> Self {
        Self {
            vertexes,
            indexes,
            normals: Vec::new(),
            texcoords: Vec::new(),
            normal_indexes: Vec::new(),
            tex_indexes: Vec::new(),
        }
    }
}
