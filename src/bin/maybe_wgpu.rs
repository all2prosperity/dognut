use std::rc::Rc;
use std::sync::Arc;
use clap::Parser;
use dognut::department::model::object_loader::ObjectLoader;
use dognut::util::Args;

fn main() {
    let arg = Args::parse();
    //
    // if arg.use_gpu {
    //     std::thread::spawn()
    // }

    let res = ObjectLoader::load_triangle_resources(&arg.obj_path);

}
