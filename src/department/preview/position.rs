use crate::department::preview::vector::HVec4;
use super::matrix::Matrix;



// #[derive(Debug, Clone)]
// pub struct Pos3 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }


pub type Pos3 = Matrix<1, 3>;

//
// impl Sub for Pos3 {
//     type Output = Vector3;
//
//     fn sub(self, other: Self) -> Vector3 {
//         let _matrix = (self.to_matrix() - other.to_matrix()).unwrap();
//         Vector3::from_matrix(_matrix)
//     }
// }


impl Default for Pos3 {
    fn default() -> Self {
        Pos3::from_vec(vec![0., 0., 0.])
    }
}

impl Pos3 {
    pub fn new_pos(x: f32, y: f32, z: f32) -> Self {
        Pos3::from_vec(vec![x, y, z])
    }

    pub fn from_matrix(matrix: &Matrix<1, 4>) -> Self {
        let (x, y, z, w) = (
            matrix.index(0, 0),
            matrix.index(0, 1),
            matrix.index(0, 2),
            matrix.index(0, 3),
        );

        Self::from_xyz(x / w, y / w, z / w)
    }

    pub fn to_homogeneous(&self) -> HVec4 {
        let _elements = vec![self.x(), self.y(), self.z(), 1.];
        Matrix::from_vec(_elements)
    }
}
