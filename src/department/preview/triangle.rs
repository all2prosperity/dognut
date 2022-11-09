use super::position::Pos3;
use super::vector::Vector3;
use super::matrix::Matrix;
use super::render_object::RenderObject;

#[derive(Debug)]
pub struct Triangle {
    pub poses: Vec<Pos3>,
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
            poses: vec![pos1, pos2, pos3]
        }
    }

    pub fn from_vec(vec: Vec<Pos3>) -> Self {
        Self {
            poses: vec
        }
    }

    pub fn get_horizon_edge(&self, y: f32, sx: u32, ex: u32) -> Option<(u32, u32)> {
        let mut target_x: Option<u32> = None;
        let mut edges: Vec<(u32, u32)> = Vec::new();
        for i in 0..3 {
            let j = if i == 2 {0} else {i + 1};
            let p1 = &self.poses[i];
            let p2 = &self.poses[j];

            let x = (p1.x + (p2.x - p1.x) * (y - p1.y) / (p2.y - p1.y)).floor() as u32;
            if x < sx || x > ex {
                continue;
            }
            
            let _sx = std::cmp::max(sx, x - 2);
            let _ex = std::cmp::min(ex, x + 2) + 1;

            let mut l = _ex + 1;
            let mut r = 0;
            for _i in _sx.._ex {
                let pos = Pos3::new(_i as f32 + 0.5, y, 0.);
                if self.in_triangle(&pos) {
                    l = std::cmp::min(l, _i);
                    r = std::cmp::max(r, _i);
                }
            }
            if l != _ex + 1 {
                edges.push((l, r));
            }
        }

        if edges.len() == 0 {
            None
        }
        else {
            let mut l = ex + 1;
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

    pub fn get_horizon_edge_deprecated(&self, y: f32, sx: u32, ex: u32) -> Option<(u32, u32)> {
        let mut target_x: Option<u32> = None;
        for i in 0..3 {
            let j = if i == 2 {0} else {i + 1};
            let p1 = &self.poses[i];
            let p2 = &self.poses[j];

            let x: f32 = p1.x + (p2.x - p1.x) * (y - p1.y) / (p2.y - p1.y);
            if x < sx as f32 || x >= ex as f32 {
                continue;
            }

            let x = x.floor() as u32 - 1;
            
            for _x in 0..3 {
                if self.in_triangle(&Pos3::new((x + _x) as f32 + 0.5, y, 0.)) {
                    target_x = Some(x + _x);
                    break;
                }
            }

            if target_x != None {
                break;
            }
        }

        if let Some(_target_x) = target_x {
            let mut l = sx;
            let mut r = _target_x;

            while l <= r {
                let mid = (l + r) / 2;
                if self.in_triangle(&Pos3::new(mid as f32 + 0.5, y, 0.)) {
                    r = mid - 1;
                }
                else {
                    l = mid + 1;
                }
            }

            let s_ret = l;

            l = _target_x;
            r = ex - 1;
            while l <= r {
                let mid = (l + r) / 2;
                if self.in_triangle(&Pos3::new(mid as f32 + 0.5, y, 0.)) {
                    l = mid + 1;
                }
                else {
                    r = mid - 1;
                }
            }

            let e_ret = r;

            Some((s_ret, e_ret))
        }
        else {
            None
        }
    }

    pub fn get_edge(&self) -> (u32, u32, u32, u32) {
        let min_x = min(min(self.poses[0].x, self.poses[1].x), self.poses[2].x);
        let max_x = max(max(self.poses[0].x, self.poses[1].x), self.poses[2].x);
        let min_y = min(min(self.poses[0].y, self.poses[1].y), self.poses[2].y);
        let max_y = max(max(self.poses[0].y, self.poses[1].y), self.poses[2].y);
        (min_x.floor() as u32, max_x.ceil() as u32, min_y.floor() as u32, max_y.ceil() as u32)
    }

    pub fn get_surface_equation(&self) -> (f32, f32, f32, f32){
        let a = (self.poses[1].y - self.poses[0].y) * (self.poses[2].z - self.poses[0].z) - (self.poses[1].z - self.poses[0].z) * (self.poses[2].y - self.poses[0].y);
        let b = (self.poses[2].x - self.poses[0].x) * (self.poses[1].z - self.poses[0].z) - (self.poses[1].x - self.poses[0].x) * (self.poses[2].z - self.poses[0].z);
        let c = (self.poses[1].x - self.poses[0].x) * (self.poses[2].y - self.poses[0].y) - (self.poses[2].x - self.poses[0].x) * (self.poses[1].y - self.poses[0].y);
        let d =  -(a * self.poses[0].x + b * self.poses[0].y + c * self.poses[0].z);
        (a, b, c, d)
    }

    pub fn get_depth_matrix(&self) -> Matrix {
        let (a, b, c, d) = self.get_surface_equation();
        Matrix::from_vec(1, 4, false, vec![-a / c, -b / c, 0., -d / c]).unwrap()
    }

    pub fn in_triangle(&self, pos: &Pos3) -> bool {
        let mut last_cross_vec: Option<Vector3> = None;
        for i in 0..3 {
            let j = if i == 2 {0} else {i + 1};
            let vec1 = &self.poses[j] - &self.poses[i];
            let vec2 = pos - &self.poses[i];
            let cross = vec2.cross(&vec1);

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
        RenderObject::from_vec(self.poses.clone(), vec![0, 1, 2])
    }
}

