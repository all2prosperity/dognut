use super::matrix;
use matrix::Matrix;


#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn to_matrix(&self) -> Matrix {
        let _elements = vec![self.x, self.y, self.z];
        Matrix::from_vec(3, 1, false, _elements).unwrap()
    }

    pub fn to_linear_matrix(&self) -> Matrix {
        let _elements = vec![self.x, self.y, self.z, 0.0];
        Matrix::from_vec(4, 1, false, _elements).unwrap()
    }

    pub fn from_matrix(matrix: Matrix) -> Self {
        Vector3 {
            x: matrix.index(0, 0),
            y: matrix.index(1, 0),
            z: matrix.index(2, 0)
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let mut _left = self.to_matrix();
        _left.transform_t();
        let _ret = (_left * other.to_matrix()).unwrap();
        _ret.index(0, 0)
    }

    pub fn to_cross_matrix(&self) -> Matrix {
        let _elements = vec![
            0., -self.z, self.y, 
            self.z, 0., -self.x,
            -self.y, self.x, 0.,
        ];
        Matrix::from_vec(3, 3, false, _elements).unwrap()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
    }

    pub fn norm(&mut self) -> &Self {
        let mag = self.magnitude();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self
    }

    pub fn to_rotation_matrix(&self, theta: f32) -> Matrix{
        let mut _clone = self.clone();
        _clone.norm();
        let n = _clone.to_matrix();

        let cos = theta.cos();
        let mut mat1 = Matrix::to_identity_matrix(3);
        mat1.mul_num(cos);

        let mut mat2 = (&n * &n.t()).unwrap();
        mat2.mul_num(1. - cos);

        let mut mat3 = _clone.to_cross_matrix();
        mat3.mul_num(theta.sin());
        let first = (&mat1 + &mat2).unwrap();
        (&first + &mat3).unwrap().add_linear()
    }
}
