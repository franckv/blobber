use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn translate(&mut self, translation: Vec3, clip: bool) {
        self.x += translation.x;
        self.y += translation.y;
        self.z += translation.z;

        if clip {
            self.x = self.x.round();
            self.y = self.y.round();
            self.z = self.z.round();
        }
    }
}

impl Into<Vec3> for Position {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
