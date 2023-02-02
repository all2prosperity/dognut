pub trait CameraTrait {
    fn update_camera(&mut self, forward_dt: f32, right_dt: f32, scroll_dt: f32, up_dt: f32, hori: f32, ver: f32, sensi: f32);

    fn to_view_position(&self) -> [f32; 4];

    fn to_view_proj(&self) -> [[f32; 4]; 4];
}
