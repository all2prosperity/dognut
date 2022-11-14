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


        Matrix::<N, M> {
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
