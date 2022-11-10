mod shader;

use std::sync::mpsc::Sender;
use crate::department::preview::matrix::Matrix;

type M44 = Matrix<4,4>;

pub struct RasterRunner<'a> {
    tx:&'a Sender<Box<Vec<char>>>,
    model_mat: M44,
    view_mat: M44,
    proj_mat: M44,

}