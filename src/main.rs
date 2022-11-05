mod department;
use department::preview::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::department::preview::vector::Vector3;

fn main_not_use() {

    let _vec = Vec::from([1., 2., 3., 4., 5., 6.]);
    let _vec2 = Vec::from([1., 1., 5., 7., 0., 3.]);

    let _d1 = matrix::Matrix::from_vec(2, 3, false, _vec).unwrap();
    let _d2 = matrix::Matrix::from_vec(3, 2, true, _vec2).unwrap();
    _d1.debug();
    _d2.debug();
    if let Some(_matrix) =  _d1.t() * _d2 {
        _matrix.debug();
        _matrix.t().debug();
    }
    else {
        println!("failed");
    }

    let _vector1 = Vector3::new(0., 0., 1.);
    let _vector3 = Vector3::new(0., 1., 0.);
    let mut v1 = Vector3::new(1.24,10.8, 9.6);
    let _ret = _vector1.cross(&_vector3);

    println!("cross result:{:#?}!", _ret);

    v1.norm();

    println!("norm is {:#?}", v1);
}

fn main() {
    pollster::block_on(state::run());
}