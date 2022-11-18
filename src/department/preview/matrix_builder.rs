use crate::department::preview::matrix::Matrix;
use crate::department::preview::vector::Vector3;

pub struct SMatrixBuilder<const M:usize>{
    row: usize,
    column: usize,
    row_vec: Option<Vec<Vector3>>,
    column_vec: Option<Vec<Vector3>>
}


impl<const M:usize> SMatrixBuilder<M> {
    pub fn new(m: usize, n:usize) -> Self{
        Self {
            row: m,
            column: n,
            row_vec: None,
            column_vec: None,
        }
    }

    pub fn add_row(mut self, r: Vector3) -> Self{

        if let Some(rv) = &mut self.row_vec {
            if rv.len() == self.row {
                return self;
            }
            rv.push(r)
        }else {
            self.row_vec = Some(vec![r]);
        }

        self
    }


    pub fn build(self) -> Matrix<M, M> {
        Matrix::new()
    }
}
