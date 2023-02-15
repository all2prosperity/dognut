use std::path::Path;

use tobj;

use crate::department::model::render_object::RenderObject;
use crate::department::model::triangle_resources::TriangleResources;
use crate::department::preview::position::Pos3;

pub struct ObjectLoader {}

impl ObjectLoader {
    pub fn load_render_obj(path: &str) -> Vec<RenderObject> {
        let _model_path = Path::new(path);
        let (models, materials) =
            tobj::load_obj(
                path,
                &tobj::LoadOptions::default(),
            )
                .expect("Failed to OBJ load file");

        // Note: If you don't mind missing the materials, you can generate a default.

        let materials = materials.unwrap_or_default();

        println!("Number of models          = {}", models.len());
        println!("Number of materials       = {}", materials.len());
        let mut render_objects: Vec<RenderObject> = Vec::new();

        for (i, m) in models.iter().enumerate() {
            let mut vertexes: Vec<Pos3> = Vec::new();
            let mut indexes: Vec<usize> = Vec::new();

            let mesh = &m.mesh;
            println!("model[{}].name             = \'{}\'", i, m.name);
            println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

            println!(
                "model[{}].face_count       = {}",
                i,
                mesh.face_arities.len()
            );
            println!("vertex_color:{:?}", mesh.vertex_color.len());
            println!("normals:{:?}", mesh.normals.len());
            println!("texcoords:{:?}", mesh.texcoords.len());
            println!("indices:{:?}", mesh.indices.len());
            // println!("vertex_color_indices:{:?}", mesh.vertex_color_indices.len());
            println!("texcoord_indices:{:?}", mesh.texcoord_indices.len());
            println!("normal_indices:{:?}", mesh.normal_indices.len());

            let mut next_face = 0;
            indexes.extend(mesh.indices.iter().map(|x| *x as usize));
            for face in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[face] as usize;

                let face_indices = &mesh.indices[next_face..end];
                if face_indices.len() != 3 {
                    // println!(" face[{}].indices          = {:?}", face, face_indices);
                } else {
                    for i in face_indices {
                        indexes.push(*i as usize);
                    }
                }
                // println!(" face[{}].indices          = {:?}", face, face_indices);

                if !mesh.texcoord_indices.is_empty() {
                    let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                    println!(
                        " face[{}].texcoord_indices = {:?}",
                        face, texcoord_face_indices
                    );
                }
                if !mesh.normal_indices.is_empty() {
                    let normal_face_indices = &mesh.normal_indices[next_face..end];
                    println!(
                        " face[{}].normal_indices   = {:?}",
                        face, normal_face_indices
                    );
                }

                next_face = end;
            }

            // Normals and texture coordinates are also loaded, but not printed in
            // this example.
            println!(
                "model[{}].positions        = {}",
                i,
                mesh.positions.len() / 3
            );
            assert!(mesh.positions.len() % 3 == 0);

            for vtx in 0..mesh.positions.len() / 3 {
                vertexes.push(Pos3::from_xyz(
                    mesh.positions[3 * vtx],
                    mesh.positions[3 * vtx + 1],
                    mesh.positions[3 * vtx + 2],
                ));
            }

            let render_o = RenderObject::from_vec(vertexes, indexes);


            render_objects.push(render_o);
        }

        for (i, m) in materials.iter().enumerate() {
            println!("material[{}].name = \'{}\'", i, m.name);
            println!(
                "    material.Ka = ({}, {}, {})",
                m.ambient[0], m.ambient[1], m.ambient[2]
            );
            println!(
                "    material.Kd = ({}, {}, {})",
                m.diffuse[0], m.diffuse[1], m.diffuse[2]
            );
            println!(
                "    material.Ks = ({}, {}, {})",
                m.specular[0], m.specular[1], m.specular[2]
            );
            println!("    material.Ns = {}", m.shininess);
            println!("    material.d = {}", m.dissolve);
            println!("    material.map_Ka = {}", m.ambient_texture);
            println!("    material.map_Kd = {}", m.diffuse_texture);
            println!("    material.map_Ks = {}", m.specular_texture);
            println!("    material.map_Ns = {}", m.shininess_texture);
            println!("    material.map_Bump = {}", m.normal_texture);
            println!("    material.map_d = {}", m.dissolve_texture);

            for (k, v) in &m.unknown_param {
                println!("    material.{} = {}", k, v);
            }
        }

        render_objects
    }

    // only load one resources for now
    pub fn load_triangle_resources(path: &str) -> TriangleResources {
        let model_path = Path::new(path);
        let (mut models, materials) =
            tobj::load_obj(
                path,
                &tobj::LoadOptions::default(),
            )
                .expect("Failed to OBJ load file");
        assert!(models.len() > 0);

        let mut mat = materials.unwrap_or_default();

        let mut triangle_resources = TriangleResources::new(models.pop().unwrap());

        let model = &triangle_resources.model;

        println!("we've got {} triangles in total.", model.mesh.indices.len() / 3);

        if let Some(i) = model.mesh.material_id {
            if model_path.is_relative() {
                let texture_path = model_path.parent().unwrap().join(Path::new(&mat[i].diffuse_texture));
                let texture = image::open(texture_path).unwrap();
                triangle_resources.image = Some(texture);
                triangle_resources.material = mat.pop();
            }
        }

        triangle_resources
    }
}
