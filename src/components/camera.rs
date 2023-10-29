#[derive(Debug)]
pub struct Camera {
    pub free_view: bool,
    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            free_view: false,
            pitch: 0.,
            yaw: 0.,
        }
    }
}
