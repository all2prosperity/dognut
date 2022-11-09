use super::matrix::Matrix;
use super::vector::Vector3;
use std::ops::{Sub};

#[derive(Debug, Clone)]
pub struct Pos3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


impl Sub for &Pos3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        let _matrix = (self.to_matrix() - other.to_matrix()).unwrap();
        Vector3::from_matrix(_matrix)
    }
}

impl Sub for Pos3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        let _matrix = (self.to_matrix() - other.to_matrix()).unwrap();
        Vector3::from_matrix(_matrix)
    }
}


impl Pos3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn from_matrix(matrix: &Matrix) -> Self {
        let (x, y, z, c) = (
            matrix.index(0, 0),
            matrix.index(1, 0),
            matrix.index(2, 0),
            matrix.index(3, 0),
            );

        if matrix.m() == 4 {
            Self {
                x: x / c,
                y: y / c,
                z: z / c,
            }
        }
        else {
            Self {x, y, z}
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        let _elements = vec![self.x, self.y, self.z, 1.];
        Matrix::from_vec(4, 1, false, _elements).unwrap()
    }
}
