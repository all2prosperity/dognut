use super::position::Pos3;
use super::vector::Vector3;
use super::matrix::Matrix;
use super::render_object::RenderObject;

#[derive(Debug)]
pub struct Triangle {
    pub v: Vec<Pos3>,
    pub color: Vec<Pos3>,
    pub normal: Vec<Pos3>,
}

pub fn max(l: f32, r: f32) -> f32{
    if l < r {
        r
    }
    else {
        l
    }
}

pub fn min(l: f32, r: f32) -> f32 {
    if l < r {
        l
    }
    else {
        r
    }
}

impl Triangle {
    pub fn new(pos1: Pos3, pos2: Pos3, pos3: Pos3) -> Self {
        Self {
            v: vec![pos1, pos2, pos3],
            color: vec![Pos3::default();3],
            normal: vec![Pos3::default();3],
        }
    }

    pub fn from_vec(vec: Vec<Pos3>) -> Self {
        Self {
            v: vec,
            color: vec![Pos3::default();3],
            normal: vec![Pos3::default();3],
        }
    }

    pub fn get_horizon_edge(&self, y: f32, sx: u32, ex: u32) -> Option<(u32, u32)> {
        //let sx = sx ;
        //let ex = ex as i32;
        let mut edges: Vec<(u32, u32)> = Vec::new();
        for i in 0..3 {
            let j = if i == 2 {0} else {i + 1};
            let p1 = &self.v[i];
            let p2 = &self.v[j];

            let x = (p1.x() + (p2.x() - p1.x()) * (y - p1.y()) / (p2.y() - p1.y())).floor() as u32;
            if x < sx as u32 || x > ex as u32 {
                continue;
            }

            let _sx = std::cmp::max(sx, x - 2);
            let _ex = std::cmp::min(ex, x + 2) + 1;

            let mut l = _ex + 1;
            let mut r = 0;
            for _i in _sx.._ex {
                let pos = Pos3::from_xyz(_i as f32 + 0.5, y, 0.);
                if self.in_triangle(&pos) {
                    l = std::cmp::min(l, _i);
                    r = std::cmp::max(r, _i);
                }
            }
            if l != _ex + 1 {
                edges.push((l as u32, r as u32));
            }
        }

        if edges.len() == 0 {
            None
        }
        else {
            let mut l = ex as u32 + 1;
            let mut r = 0;

            for (_l, _r) in edges {
                l = std::cmp::min(l, _l);
                l = std::cmp::min(l, _r);

                r = std::cmp::max(r, _l);
                r = std::cmp::max(r, _r);
            }

            Some((l, r))
        }
    }

    pub fn get_edge(&self) -> (u32, u32, u32, u32) {
        let min_x = min(min(self.v[0].x(), self.v[1].x()), self.v[2].x());
        let max_x = max(max(self.v[0].x(), self.v[1].x()), self.v[2].x());
        let min_y = min(min(self.v[0].y(), self.v[1].y()), self.v[2].y());
        let max_y = max(max(self.v[0].y(), self.v[1].y()), self.v[2].y());
        (min_x.floor() as u32, max_x.ceil() as u32, min_y.floor() as u32, max_y.ceil() as u32)
    }

    pub fn get_surface_equation(&self) -> (f32, f32, f32, f32){
        let a = (self.v[1].y() - self.v[0].y()) * (self.v[2].z() - self.v[0].z()) - (self.v[1].z() - self.v[0].z()) * (self.v[2].y() - self.v[0].y());
        let b = (self.v[2].x() - self.v[0].x()) * (self.v[1].z() - self.v[0].z()) - (self.v[1].x() - self.v[0].x()) * (self.v[2].z() - self.v[0].z());
        let c = (self.v[1].x() - self.v[0].x()) * (self.v[2].y() - self.v[0].y()) - (self.v[2].x() - self.v[0].x()) * (self.v[1].y() - self.v[0].y());
        let d =  -(a * self.v[0].x() + b * self.v[0].y() + c * self.v[0].z());
        (a, b, c, d)
    }

    pub fn get_depth_matrix(&self) -> Matrix<1,4> {
        let (a, b, c, d) = self.get_surface_equation();
        Matrix::from_vec( vec![-a / c, -b / c, 0., -d / c])
    }

    pub fn in_triangle(&self, pos: &Pos3) -> bool {
        let mut last_cross_vec: Option<Vector3> = None;
        for i in 0..3 {
            let j = if i == 2 {0} else {i + 1};
            let vec1 = &self.v[j] - &self.v[i];
            let vec2 = pos - &self.v[i];
            let cross = vec2.cross(&vec1);

            //todo: 假定三角形顶点是逆时针定义的，那么只要有一个cross product的z为负值，那么就可以判定在三角形外
            // println!("cur last is {:?}", last_cross_vec);
            if let Some(_last_cross_vec) = &last_cross_vec {
                if _last_cross_vec.dot(&cross) < 0. {
                    return false;
                }
            }

            last_cross_vec = Some(cross);
        }

        true
    }

    pub fn to_render_obj(&self) -> RenderObject {
        RenderObject::from_vec(self.v.clone(), vec![0, 1, 2])
    }
}

impl Triangle {
    pub fn get_color(&self, x: usize, y:usize) -> Vector3 {
        return Vector3::from_xyz(0., 0., 0.);
    }

    pub fn get_normal(&self, x: usize, y:usize) -> Vector3 {
        return Vector3::from_xyz(0., 0., 0.);
    }
}

