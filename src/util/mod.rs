use clap::Parser;

mod cmd_arg;

/// render a object to window or terminal
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// whether use gpu or cpu simulated renderer.
    #[arg(short, long)]
    use_gpu: bool,

    /// gui or terminal mode
    #[arg(short, long)]
    term: bool,

    /// object path to load, only support triangulated obj.
    #[arg(long)]
    obj_path: String,

    /// only render a jpeg picture
    #[arg(short, default_value_t=false)]
    render_a_picture: bool,
}