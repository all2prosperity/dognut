use crate::department::preview::matrix::Matrix;
use crate::department::preview::vector::Vector3;

pub type HomoTransform = Matrix<4,4>;

pub type Transform = Matrix<4,4>;

// for left multiplication
impl HomoTransform {
    pub fn translation(to:(f32, f32, f32)) -> Self {
        Self::from_vec(
            vec![
                1., 0., 0., 0.,
                0., 1., 0., 0.,
                0., 0., 1., 0.,
                to.0, to.1, to.2, 1.,
            ]
        )
    }

    pub fn rotation_matrix(n: &Vector3, theta: f32) -> Self{
        let mut vt = n.clone();
        let v = vt.norm();
        let (sin_t, cos_t) = theta.sin_cos();

        let (x, y, z) = (v.x(), v.y(), v.z());
        let cminus1 = 1. - cos_t;

        HomoTransform::from_vec(vec![
            x.powi(2) * cminus1 + cos_t, x * y * cminus1 - z * sin_t, x * z * cminus1 + y * sin_t,0.,
            x * y * cminus1 + z * sin_t, y.powi(2) * cminus1 + cos_t, y * z * cminus1 - x * sin_t,0.,
            x * z * cminus1 - y * sin_t, y * z * cminus1 + x*sin_t, z.powi(2) * cminus1  + cos_t,0.,
            0.,0.,0.,1.
        ]
        )
    }
}

impl Transform {
    pub fn rotation_mat(n: &Vector3, theta: f32) -> Self{
        let mut vt = n.clone();
        let v = vt.norm();
        let (sin_t, cos_t) = theta.sin_cos();

        let (x, y, z) = (v.x(), v.y(), v.z());
        let cminus1 = 1. - cos_t;

        Transform::from_vec(vec![
            x.powi(2) * cminus1 + cos_t, x * y * cminus1 - z * sin_t, x * z * cminus1 + y * sin_t, 0.,
            x * y * cminus1 + z * sin_t, y.powi(2) * cminus1 + cos_t, y * z * cminus1 - x * sin_t, 0.,
            x * z * cminus1 - y * sin_t, y * z * cminus1 + x*sin_t, z.powi(2) * cminus1  + cos_t, 0.,
            0., 0., 0., 1.
        ])
    }
}