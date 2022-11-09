use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Matrix {
    // 0 1 2  m = 2, n = 3
// 0 1 2
//
//
    m: usize,
    n: usize,
    t: bool,
    elements: Vec<f32>,
}

pub struct MatrixIter<'a> {
    iter: &'a Matrix,
    x: usize,
    y: usize,
}

impl<'a> MatrixIter<'a> {
    fn new(iter: &'a Matrix, x: usize, y: usize) -> Self {
        Self {
            iter,
            x,
            y,
        }
    }
}

impl Mul for Matrix {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Option<Self> {
        if self.n() != other.m() {
            None
        } else {
            let _m = self.m();
            let _n = other.n();
            let _common_len = self.n();
            let mut _ret = Matrix::new(self.m(), other.n(), false);
            for i in 0.._m {
                for j in 0.._n {
                    let mut _val = 0.;
                    for k in 0.._common_len {
                        _val += self.index(i, k) * other.index(k, j);
                    }

                    _ret.set(i, j, _val);
                }
            }

            Some(_ret)
        }
    }
}

impl Mul for &Matrix {
    type Output = Option<Matrix>;

    fn mul(self, other: Self) -> Option<Matrix> {
        if self.n() != other.m() {
            None
        } else {
            let _m = self.m();
            let _n = other.n();
            let _common_len = self.n();
            let mut _ret = Matrix::new(self.m(), other.n(), false);
            for i in 0.._m {
                for j in 0.._n {
                    let mut _val = 0.;
                    for k in 0.._common_len {
                        _val += self.index(i, k) * other.index(k, j);
                    }

                    _ret.set(i, j, _val);
                }
            }

            Some(_ret)
        }
    }
}

impl Add for &Matrix {
    type Output = Option<Matrix>;

    fn add(self, other: Self) -> Option<Matrix> {
        if self.m() != other.m() {
            None
        } else if self.n() != other.n() {
            None
        } else {
            let mut _s_iter = self.iter();
            let mut _o_iter = other.iter();
            let mut _elements = Vec::new();
            loop {
                if let Some(_item) = _s_iter.next() {
                    _elements.push(_item + _o_iter.next().unwrap())
                } else {
                    break;
                }
            }
            Matrix::from_vec(self.m(), self.n(), false, _elements)
        }
    }
}


impl Add for Matrix {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.m() != other.m() {
            None
        } else if self.n() != other.n() {
            None
        } else {
            let mut _s_iter = self.iter();
            let mut _o_iter = other.iter();
            let mut _elements = Vec::new();
            loop {
                if let Some(_item) = _s_iter.next() {
                    _elements.push(_item + _o_iter.next().unwrap())
                } else {
                    break;
                }
            }
            Matrix::from_vec(self.m(), self.n(), false, _elements)
        }
    }
}

impl Sub for Matrix {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Option<Self> {
        if self.m() != other.m() {
            None
        } else if self.n() != other.n() {
            None
        } else {
            let mut _s_iter = self.iter();
            let mut _o_iter = other.iter();
            let mut _elements = Vec::new();
            loop {
                if let Some(_item) = _s_iter.next() {
                    _elements.push(_item - _o_iter.next().unwrap())
                } else {
                    break;
                }
            }
            Matrix::from_vec(self.m(), self.n(), false, _elements)
        }
    }
}

impl Matrix {
    pub fn new(m: usize, n: usize, t: bool) -> Self {
        let mut _elements: Vec<f32> = Vec::with_capacity((m * n) as usize);
        _elements.resize((m * n) as usize, 0.0);
        Self {
            m,
            n,
            t,
            elements: _elements,
        }
    }

    pub fn from_vec(m: usize, n: usize, t: bool, elements: Vec<f32>) -> Option<Self> {
        if m * n != elements.len() {
            None
        } else {
            Some(Self {
                m,
                n,
                t,
                elements,
            })
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

    pub fn t(&self) -> Self {
        let mut transposed_elems = self.elements.clone();
        let _n = self.m;
        let _m = self.n;

        for i in 0.._m {
            for j in 0.._n {
                transposed_elems[i * _n + j] = self.elements[j*self.n + i];
            }
        }


        Matrix {
            m: _m,
            n: _n,
            t: false,
            elements: transposed_elems
        }
    }

    pub fn transform_t(&mut self) {
        self.t = self.t;
    }

    pub fn iter(&self) -> MatrixIter {
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

    pub fn to_identity_matrix(num: usize) -> Self {
        let mut ret = Self::new(num, num, false);

        for i in 0..num {
            ret.set(i, i, 1.)
        }
        ret
    }

    pub fn mul_num(&mut self, num: f32) -> &Self {
        for i in self.elements.iter_mut() {
            *i *= num;
        }
        self
    }

    pub fn add_linear(&self) -> Self {
        let mut ret = Matrix::new(self.m() + 1, self.n() + 1, false);

        for i in 0..self.m() {
            for j in 0..self.n() {
                ret.set(i, j, self.index(i, j));
            }
        }

        ret.set(ret.m() - 1, ret.n() - 1, 1.);
        ret
    }

    pub fn move_matrix(x: f32, y: f32, z: f32) -> Self {
        Self::from_vec(4, 4, false, vec![
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.,
        ]).unwrap()
    }
}

impl<'a> Iterator for MatrixIter<'a> {
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
