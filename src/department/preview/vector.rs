use std::borrow::BorrowMut;
use super::matrix::Matrix;

pub type Vector3 = Matrix<1,3>;

impl Vector3 {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Vector3::from_vec(vec![x,y,z])
    }

    pub fn x(&self) -> f32 {
        self.elements[0]
    }
    pub fn y(&self) -> f32 {
        self.elements[1]
    }
    pub fn z(&self) -> f32 {
        self.elements[2]
    }

    pub fn to_linear_matrix(&self) -> Matrix::<1,4> {
        let  mut ele = self.elements.clone();
        ele.push(0.);
        Matrix::<1,4>::from_vec(ele)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let mut res = 0f32;
        for i in 0..self.n {
            res += self.elements[i] * other.elements[i];
        }
        res
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::from_xyz(self.y() * other.z() - self.z() * other.y(),
                       self.z() * other.x() - self.x() * other.z(),
                       self.x() * other.y() - self.y() * other.x())
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(f32::powi(self.x(), 2) + f32::powi(self.y(), 2) + f32::powi(self.z(), 2))
    }

    pub fn norm(&mut self) -> &Self {
        let mag = self.magnitude();
        self.elements[0] = self.elements[0] / mag;
        self.elements[1] = self.elements[1] / mag;
        self.elements[2] = self.elements[2] / mag;
        self
    }

    // pub fn to_rotation_matrix(&self, theta: f32) -> Self{
    //     let mut _clone = self.clone();
    //     _clone.norm();
    //     let n = _clone.to_matrix();
    //
    //     let cos = theta.cos();
    //     let mut mat1 = Matrix::to_identity_matrix(3);
    //     mat1.mul_num(cos);
    //
    //     let mut mat2 = (&n * &n.t()).unwrap();
    //     mat2.mul_num(1. - cos);
    //
    //     let mut mat3 = _clone.to_cross_matrix();
    //     mat3.mul_num(theta.sin());
    //     let first = (&mat1 + &mat2).unwrap();
    //     (&first + &mat3).unwrap().add_linear()
    // }
}
