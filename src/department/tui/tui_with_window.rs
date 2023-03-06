use std::io::Stdout;
use crate::department::common::self_type;
use crate::department::control::camera_controller::CameraController;
use crate::department::pipeline::rasterizer::RasterRunner;
use crate::department::types::msg::TransferMsg;
use crate::department::types::multi_sender::MultiSender;

pub struct TuiWinApp {
    pub raster: RasterRunner,
    stdout: Stdout,
    theta: f32,
    camera_controller: CameraController,
    gpu: Option<self_type::StateImp>,
    fps: u32,
    time_step: std::time::Duration,
    res: crate::department::model::triangle_resources::TriangleResources,
    ms: MultiSender<TransferMsg>
}
