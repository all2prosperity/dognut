use super::triangle::Triangle;
use super::render_object::RenderObject;
use super::position::Pos3;

#[derive(Debug)]
pub struct ObjectBuffer {
    pub object_list: Vec<RenderObject>
}


pub struct ObjectBufferIter<'a> {
    iter: &'a ObjectBuffer,
    obj_idx: usize,
    idx_idx: usize,
}


impl<'a> ObjectBufferIter<'a> {
    fn new(iter: &'a ObjectBuffer) -> Self {
        Self {
            iter,
            obj_idx: 0,
            idx_idx: 0,
        }
    }
}

impl<'a> Iterator for ObjectBufferIter<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.obj_idx >= self.iter.object_list.len() {
            None
        }
        else if self.idx_idx >= self.iter.object_list[self.obj_idx].indexes.len() {
            None
        }
        else {
            let render_obj = &self.iter.object_list[self.obj_idx];
            let mut pos_vec: Vec<Pos3> = Vec::new();
            for _ in 0..3 {
                let _pos = &render_obj.vertexes[render_obj.indexes[self.idx_idx]];
                pos_vec.push(_pos.clone());
                self.idx_idx += 1;
            }

            if self.idx_idx >= render_obj.indexes.len() {
                self.idx_idx = 0;
                self.obj_idx += 1;
            }

            Some(Triangle::from_vec(pos_vec))
        }
    }
}


impl ObjectBuffer {
    pub fn new() -> Self {
        Self {
            object_list: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: RenderObject) -> &mut Self {
        self.object_list.push(obj);
        self
    }

    pub fn iter(&self) -> ObjectBufferIter {
        ObjectBufferIter::new(self)
    }
}
