use clap::Parser;

pub mod cmd_arg;

/// render a object to window or terminal
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// whether use gpu or cpu simulated renderer.
    #[arg(short, long, default_value_t=false)]
    pub use_gpu: bool,

    /// gui or terminal mode
    #[arg(short, long, default_value_t=false)]
    pub term: bool,

    /// object path to load, only support triangulated obj.
    #[arg(long, default_value_t=String::from("./res/cube/cube.obj"))]
    pub obj_path: String,

    /// only render a jpeg picture
    #[arg(short, default_value_t=false)]
    pub render_a_picture: bool,
}