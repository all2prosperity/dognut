use image::{DynamicImage, GenericImageView};
use tobj::{Material, Model};

use crate::department::model::triangle::Triangle;
use crate::department::preview::vector::{Vec2, Vector3};

pub struct TriangleIter<'a> {
    pub resources: &'a TriangleResources,
    pub triangle_idx: usize,
    max_idx: usize,
}


impl<'a> Iterator for TriangleIter<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.triangle_idx >= self.max_idx  {
            return None;
        }


        let (width ,height) = self.resources.image.as_ref().unwrap().dimensions();
        let (width, height) = (width - 1, height - 1);

        let m = &self.resources.model.mesh;
        let mut points = Vec::<Vector3>::new();
        let mut normals = Vec::<Vector3>::new();
        let mut tex_coords = Vec::<Vec2>::new();

        for i in self.triangle_idx..self.triangle_idx + 3 {
            let pi= m.indices[i] as usize;
            let ni = m.normal_indices[i] as usize;
            let ti = m.texcoord_indices[i] as usize;

            points.push(Vector3::from_xyz(
                m.positions[pi*3],
                m.positions[pi*3 + 1],
                m.positions[pi*3 + 2],
            ));
            normals.push(Vector3::from_xyz(
                m.normals[ni*3],
                m.normals[ni*3 + 1],
                m.normals[ni*3 + 2],
            ));

            tex_coords.push(Vec2::from_xy(
                m.texcoords[ti*2] * width as f32,
                height as f32 - m.texcoords[ti*2 + 1] * height as f32,
            ))
        }
        self.triangle_idx += 3;



        // let mut colors = Vec::<Vector3>::new();
        // if let Some(img) = &self.resources.image {
        //     let (width ,height) = img.dimensions();
        //     let (width, height) = (width - 1, height - 1);
        //     for tex_coord in &tex_coords {
        //         let c = img.get_pixel((tex_coord.u() * width as f32) as u32, (tex_coord.v() * height as f32) as u32);
        //         colors.push(Vector3::from_xyz(c.0[0] as f32 , c.0[1] as f32 , c.0[2] as f32 ));
        //     }
        // }

        let tri = Triangle::from_mesh_vec(points, normals, tex_coords);
        // tri.set_color_row(colors);
        Some(tri)
    }
}

pub struct TriangleResources {
    pub model: Model,
    pub material: Option<Material>,
    pub image: Option<DynamicImage>,
}


impl TriangleResources {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            material: None,
            image: None,
        }
    }

    pub fn iter(&self) -> TriangleIter {
        let max = self.model.mesh.indices.len();
        TriangleIter{
            resources: self,
            triangle_idx: 0,
            max_idx: max
        }
    }
}
