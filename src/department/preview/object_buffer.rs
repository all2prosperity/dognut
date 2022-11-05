use super::position::Pos3;
use super::vector::Vector3;

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

    pub fn get_edge(&self) -> (u32, u32, u32, u32) {
        let min_x = min(min(self.poses[0].x, self.poses[1].x), self.poses[2].x);
        let max_x = max(max(self.poses[0].x, self.poses[1].x), self.poses[2].x);
        let min_y = min(min(self.poses[0].y, self.poses[1].y), self.poses[2].y);
        let max_y = max(max(self.poses[0].y, self.poses[1].y), self.poses[2].y);
        (min_x.floor() as u32, max_x.ceil() as u32, min_y.floor() as u32, max_y.ceil() as u32)
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
}

pub struct ObjectBuffer {
    pub
    buffer: Vec<Triangle>   
}

impl ObjectBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn add_object(&mut self, obj: Triangle) -> &mut Self {
        self.buffer.push(obj);
        self
    }
}
