use super::position::Pos3;
use super::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct RenderObject {
    pub vertexes: Vec<Pos3>,
    pub indexes: Vec<usize>,
}

impl RenderObject {
    pub fn new() -> Self {
        Self {
            vertexes: Vec::new(),
            indexes: Vec::new(),
        }
    }

    pub fn mul_matrix(&mut self, mat: &Matrix) {
        for i in self.vertexes.iter_mut() {
            *i = Pos3::from_matrix(&(mat * &i.to_matrix()).unwrap());
        }
    }

    pub fn from_vec(vertexes: Vec<Pos3>, indexes: Vec<usize>) -> Self {
        Self {
            vertexes,
            indexes,
        }
    }
}
