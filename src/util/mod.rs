use clap::Parser;

pub mod cmd_arg;

use lazy_static::lazy_static;


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
    #[arg(long, default_value_t=String::from("./res/plane/plane.obj"))]
    pub obj_path: String,

    /// only render a jpeg picture
    #[arg(short, default_value_t=false)]
    pub render_a_picture: bool,
}


lazy_static!{
    pub static ref ARG: Args = Args::parse();
}

pub fn split_screen(data: &Vec<u8>, original_dimension: (u32, u32), split_dimension: (u32, u32)) -> (Vec<u8>, Vec<u8>) {
    let skip = 4u32;
    let mut left = Vec::with_capacity((split_dimension.0 * split_dimension.1 * skip) as usize);
    let mut right = Vec::with_capacity(((original_dimension.0 - split_dimension.0) * split_dimension.1 * skip) as usize);


    for row in 0..original_dimension.1 {
        let row_start = (row * original_dimension.0 * skip);
        let left_copy_start = row_start as usize;
        let right_copy_start = (row_start + split_dimension.0 * skip) as usize;
        let line_end = ((row + 1) * original_dimension.0 * skip) as usize;
        left.extend_from_slice(&data[left_copy_start..right_copy_start]);
        right.extend_from_slice(&data[right_copy_start..line_end]);
    }

    return (left, right);
}
