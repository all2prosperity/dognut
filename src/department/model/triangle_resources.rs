use image::{DynamicImage, GenericImageView};
use tobj::{Material, Model};
use crate::department::model::triangle::Triangle;
use crate::department::preview::vector::{Vec2, Vector3};


pub struct TriangleIter<'a> {
    pub resources: &'a TriangleResources,
    pub triangle_idx: usize,
}


impl<'a> Iterator for TriangleIter<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.triangle_idx >= (&self.resources.model.mesh.indices.len() / 3) {
            return None;
        }


        let m = &self.resources.model.mesh;
        let mut points = Vec::<Vector3>::new();
        let mut normals = Vec::<Vector3>::new();
        let mut tex_coords = Vec::<Vec2>::new();

        for i in self.triangle_idx..self.triangle_idx + 3 {
            let pi= m.indices[i] as usize;
            let ni = m.normal_indices[i] as usize;
            let ti = m.texcoord_indices[i] as usize;

            points.push(Vector3::from_xyz(
                m.positions[pi],
                m.positions[pi + 1],
                m.positions[pi + 2],
            ));
            normals.push(Vector3::from_xyz(
                m.normals[ni],
                m.normals[ni + 1],
                m.normals[ni + 2],
            ));

            tex_coords.push(Vec2::from_xy(
                m.texcoords[ti],
                m.texcoords[ti + 1],
            ))
        }
        self.triangle_idx += 3;

        let mut colors = Vec::<Vector3>::new();
        if let Some(img) = &self.resources.image {
            let (width ,height) = img.dimensions();
            for tex_coord in &tex_coords {
                let c = img.get_pixel((tex_coord.u() * width as f32) as u32, (tex_coord.v() * height as f32) as u32);
                colors.push(Vector3::from_xyz(c.0[0] as f32 / 255., c.0[1] as f32 / 255., c.0[2] as f32 /255.));
            }
        }

        let mut tri = Triangle::from_mesh_vec(points, normals, tex_coords);
        tri.set_color_row(colors);
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
        TriangleIter{
            resources: self,
            triangle_idx: 0,
        }
    }
}