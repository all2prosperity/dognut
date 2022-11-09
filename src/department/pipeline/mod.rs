mod shader;

use std::sync::mpsc::Sender;
use crate::department::preview::matrix::Matrix;

pub struct RasterRunner<'a> {
    tx:&'a Sender<Box<Vec<char>>>,
    model_mat: Matrix,
    view_mat: Matrix,
    proj_mat: Matrix,

}