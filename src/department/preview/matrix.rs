use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Debug, Clone)]
pub struct Matrix<const M: usize, const N: usize> {
    // 0 1 2  m = 2, n = 3
// 0 1 2
//
//
    pub m: usize,
    pub n: usize,
    pub elements: Vec<f32>,
}

pub type HMat = Matrix<4,4>;

pub struct MatrixIter<'a, const M: usize, const N: usize> {
    iter: &'a Matrix<M, N>,
    x: usize,
    y: usize,
}

impl<'a, const M: usize, const N: usize> MatrixIter<'a, M, N> {
    fn new(iter: &'a Matrix<M, N>, x: usize, y: usize) -> Self {
        Self {
            iter,
            x,
            y,
        }
    }
}

impl<const M: usize, const N: usize, const K: usize> Mul<Matrix<N, K>> for Matrix<M, N> {
    type Output = Matrix<M, K>;

    fn mul(self, other: Matrix<N, K>) -> Matrix<M, K> {
        let _m = self.m();
        let _n = other.n();
        let _common_len = self.n();
        let mut _ret = Matrix::<M, K>::new();
        for i in 0.._m {
            for j in 0.._n {
                let mut _val = 0.;
                for k in 0.._common_len {
                    _val += self.index(i, k) * other.index(k, j);
                }

                _ret.set(i, j, _val);
            }
        }
        _ret
    }
}


impl<const M: usize, const N: usize, const K: usize> Mul<&Matrix<N, K>> for &Matrix<M, N> {
    type Output = Matrix<M, K>;

    fn mul(self, other: &Matrix<N, K>) -> Self::Output {
        let _m = self.m();
        let _n = other.n();
        let _common_len = self.n();
        let mut _ret = Matrix::<M, K>::new();
        for i in 0.._m {
            for j in 0.._n {
                let mut _val = 0.;
                for k in 0.._common_len {
                    _val += self.index(i, k) * other.index(k, j);
                }

                _ret.set(i, j, _val);
            }
        }
        _ret
    }
}

impl<const M: usize, const N: usize> Mul<f32> for Matrix<M, N> {
    type Output = Matrix<M,N>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::from_vec(self.elements.iter().map(|f| f*rhs).collect())
    }
}

impl<const M: usize, const N: usize> MulAssign<f32> for Matrix<M, N> {
    fn mul_assign(&mut self, rhs: f32) {
        self.elements.iter_mut().for_each(|f| *f *= rhs);
    }
}


impl<const M: usize, const N: usize> AddAssign<f32> for Matrix<M, N> {
    fn add_assign(&mut self, rhs: f32) {
        self.elements.iter_mut().for_each(|f| *f += rhs);
    }
}

impl<const M: usize, const N: usize> AddAssign<f32> for &mut Matrix<M, N> {
    fn add_assign(&mut self, rhs: f32) {
        self.elements.iter_mut().for_each(|f| *f += rhs);
    }
}

impl<const M: usize, const N: usize> Add for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Matrix::<M, N>::new();
        let mut i = 0;
        ret.elements.iter_mut().for_each(|f| {
            *f = self.elements[i] + rhs.elements[i];
            i += 1;
        });
        ret
    }
}

impl<const M: usize, const N: usize> Add for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Matrix::<M, N>::new();
        let mut i = 0;
        ret.elements.iter_mut().for_each(|f| {
            *f = self.elements[i] + rhs.elements[i];
            i += 1;
        });
        ret
    }
}

impl<const M: usize, const N: usize> Sub for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Matrix::<M, N>::new();
        let mut i = 0;
        ret.elements.iter_mut().for_each(|f| {
            *f = self.elements[i] - rhs.elements[i];
            i += 1;
        });
        ret
    }
}

impl<const M: usize, const N: usize> Sub for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Matrix::<M, N>::new();
        let mut i = 0;
        ret.elements.iter_mut().for_each(|f| {
            *f = self.elements[i] - rhs.elements[i];
            i += 1;
        });
        ret
    }
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new() -> Self {


        let mut _elements: Vec<f32> = Vec::with_capacity((M * N) as usize);
        _elements.resize((M * N) as usize, 0.0);
        Self {
            m: M,
            n: N,
            elements: _elements,
        }
    }

    pub fn from_vec(elements: Vec<f32>) -> Self {
        Self {
            m:M,
            n:N,
            elements,
        }
    }

    pub fn cut<const SUBM: usize, const SUBN: usize>(&self, x: usize, y: usize) -> Matrix<SUBM, SUBN> {
        let mut ret = Matrix::<SUBM, SUBN>::new();
        for i in 0..SUBM {
            for j in 0..SUBN {
                ret.set(i, j, self.index(x + i, y + j));
            }
        }

        ret
    }

    pub fn paste<const SUBM: usize, const SUBN: usize>(&mut self, other: &Matrix<SUBM, SUBN>, x: usize, y: usize) {
        for i in 0..SUBM {
            for j in 0..SUBN {
                self.set(x + i, y + j, other.index(i, j));
            }
        }
    }

    pub fn debug(&self) {
        let mut _print = String::from("");
        for i in 0..(self.m()) {
            for j in 0..(self.n()) {
                _print.push(' ');
                _print += &self.index(i, j).to_string();
            }

            _print.push('\n');
        }

        println!("{}", _print);
    }

    pub fn m(&self) -> usize {
        self.m
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn t(&self) -> Matrix<N, M> {
        let mut transposed_elems = self.elements.clone();
        let _n = self.m;
        let _m = self.n;

        for i in 0.._m {
            for j in 0.._n {
                transposed_elems[i * _n + j] = self.elements[j * self.n + i];
            }
        }


        Matrix {
            m: _m,
            n: _n,
            elements: transposed_elems,
        }
    }

    pub fn transform_t(&mut self) {}

    pub fn iter(&self) -> MatrixIter<M, N> {
        MatrixIter::new(self, 0, 0)
    }

    pub fn set(&mut self, x: usize, y: usize, val: f32) {
        self.elements[x * self.n + y] = val;
    }

    pub fn index(&self, x: usize, y: usize) -> f32 {
        self.elements[x * self.n + y]
    }

    pub fn result(&self) -> f32 {
        self.elements[0]
    }

    pub fn mul_num(&mut self, num: f32) -> &Self {
        for i in self.elements.iter_mut() {
            *i *= num;
        }
        self
    }

    pub fn add_linear(&self) -> Self {
        let mut ret = Matrix::new();

        for i in 0..self.m() {
            for j in 0..self.n() {
                ret.set(i, j, self.index(i, j));
            }
        }

        ret.set(ret.m() - 1, ret.n() - 1, 1.);
        ret
    }

    pub fn translate_matrix(x: f32, y: f32, z: f32) -> Self {
        Self::from_vec(vec![
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.,
        ])
    }
}

// only for square matrix
impl<const M: usize> Matrix<M, M> {
    pub fn identity_matrix() -> Self {
        let mut ret = Self::new();

        for i in 0..M {
            ret.set(i, i, 1.)
        }
        ret
    }

    pub fn upper_triangular_matrix_inverse(&self) -> Self
            where [(); M / 2]:,
                [(); M - M / 2]:, {
        if M == 1 {
            return Self::from_vec(vec![1. / self.index(0, 0)]);
        }

        // const small: usize = M / 2;
        // let big = M - small;
        
        let left_up = self.cut::<{M / 2}, {M / 2}>(0, 0);
        let right_bottom = self.cut::<{M - M / 2}, {M - M / 2}>(M / 2, M / 2);
        let up_right = self.cut::<{M / 2}, {M - M / 2}>(0, M / 2);

        let left_up_1 = left_up.upper_triangular_matrix_inverse();
        let right_bottom_1 = right_bottom.upper_triangular_matrix_inverse();
        let mut ret = Self::new();
        ret.paste(&left_up_1, 0, 0);

        ret
    }

    pub fn inverse_matrix(&self) 
            where [(); M / 2]:,
                [(); M - M / 2]:, 
    {
        let mut es: Vec<Self> = Vec::new();
        let mut us: Vec<Self> = Vec::new();
        let mut u = self.clone();
        let mut l = Self::identity_matrix();
        let mut l_1 = Self::identity_matrix();
        for i in 1..M {
            for j in 0..i {
                let mut e = Self::identity_matrix();
                let mut divisor = u.index(j, j);
                let mut dividend = 0.;

                for k in 0..M {
                    if k != j {
                        dividend += e.index(i, k) * u.index(k, j);
                    } 
                }

                let val = -dividend / divisor;
                e.set(i, j, val);

                u = &e * &u;
                println!("cur is {}, {}:", i + 1, j);
                e.debug();
                u.debug();
                l_1 = &e * &l_1;

                let mut _tmp = Self::identity_matrix();
                _tmp.set(i, j, -val);
                l = &l * &_tmp;
            }
        }

        u.upper_triangular_matrix_inverse();

        println!("end:");
        l.debug();
        l_1.debug();
        u.debug();
        let ret = l * l_1;
        ret.debug();
    }
}

impl<'a, const M: usize, const N: usize> Iterator for MatrixIter<'a, M, N> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.iter.m() {
            None
        } else {
            let _ret = self.iter.index(self.x, self.y);

            self.y += 1;
            if self.y == self.iter.n() {
                self.y = 0;
                self.x += 1
            }

            Some(_ret)
        }
    }
}


