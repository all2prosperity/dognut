use std::ops::{AddAssign, SubAssign};

use super::matrix::{HMat, Matrix};

pub type Vec2 = Matrix<1, 2>;


pub type HVec4 = Matrix<1,4>;


impl Default for Vec2 {
    fn default() -> Self {
        Vec2{
            m:1,
            n:2,
            elements: vec![0., 0.]
        }
    }
}

impl Vec2 {
    pub fn from_xy(x: f32, y:f32) -> Self {
        Vec2{
            m:1,
            n:2,
            elements: vec![x,y]
        }
    }

    pub fn u(&self) -> f32 {
        self.elements[0]
    }

    pub fn v(&self) -> f32 {
        self.elements[1]
    }
}

pub type Vector3 = Matrix<1,3>;

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        for i  in 0..3 {
            self.elements[i] += rhs.elements[i];
        }
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Self) {
        for i  in 0..3 {
            self.elements[i] += rhs.elements[i];
        }
    }
}

impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        for i in 0..3 {
            self.elements[i] -= rhs.elements[i];
        }
    }
}

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

    pub fn to_rotate_negative_z_matrix(&self, up: &Self) -> HMat{
        let mut fwd = self.clone();
        fwd.norm();
        
        let w =  fwd * -1f32;
        let mut u = up.cross(&self);
        u.norm();
        let v = w.cross(&u);

        HMat::from_vec( vec![
            u.x(), v.x(), w.x(), 0.,
            u.y(), v.y(), w.y(), 0.,
            u.z(), v.z(), w.z(), 0.,
            0., 0., 0., 1.,
        ])
    }
}

impl HVec4 {
    pub fn from_v3(v: Vector3) -> Self {
        let mut ele = v.elements.clone();
        ele.push(1.);
        Self {
            m:1,
            n:4,
            elements: ele,
        }
    }



    pub fn persp_divide(&mut self){
        for i in 0..self.n {
            self.elements[i] /= self.elements[3];
        }
    }

    pub fn x(&self) -> f32{
        self.elements[0]
    }

    pub fn y(&self) -> f32{
        self.elements[1]
    }

    pub fn z(&self) -> f32{
        self.elements[2]
    }

    pub fn w(&self) -> f32{
        self.elements[3]
    }
}
