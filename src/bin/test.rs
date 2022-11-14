use dognut::department::preview::matrix::Matrix;
//use dognut::department::preview::position::Pos3;
//use dognut::department::preview::vector::Vector3;

fn main() {
    let _vec = Vec::from([1., 2., 3., 4., 5., 6.]);
    let _vec2 = Vec::from([1., 1., 5., 7., 0., 3.]);

    let _d1 = Matrix::<2,3>::from_vec( _vec);
    let _d2 = Matrix::<2,3>::from_vec( _vec2);
    _d1.debug();
    _d2.debug();
    let mut d1_t = _d1.t();
    d1_t.debug();
    let d = &d1_t * &_d2;
    d.debug();

    d1_t  *= 3.;
    d1_t.debug();


    let origin = Matrix::<3, 3>::from_vec(vec![
        1., 2., 3.,
        4., 5., 6.,
        7., 8., 10.,
    ]);

    origin.inverse_matrix();

    // let _vector1 = Vector3::new(0., 0., 1.);
    // let _vector3 = Vector3::new(0., 1., 0.);
    // let mut v1 = Vector3::new(1.24,10.8, 9.6);
    // let _ret = _vector1.cross(&_vector3);
    //
    // println!("cross result:{:#?}!", _ret);
    //
    // v1.norm();
    //
    // println!("norm is {:#?}", v1);
    //
    // let vertical = Vector3::new(0., 1., 0.);
    // let theta = std::f32::consts::PI / 2.;
    // let rotate = vertical.to_rotation_matrix(theta);
    //
    // let pos = Pos3::new(5., 6., 7.);
    // let ret = (&rotate * &pos.to_matrix()).unwrap();
    // let pos = Pos3::from_matrix(&ret);
    //
    // println!("rotate :{:?}, pos:{:?}, sin:{:?}", ret, pos, theta.sin());
}
