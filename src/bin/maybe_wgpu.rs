use dognut::department::model::object_loader::ObjectLoader;

fn main() {
    let res = ObjectLoader::load_triangle_resources("./model/Link/link_adult.obj");
    for t in res.iter() {
        t.color.unwrap().debug();
    }
}