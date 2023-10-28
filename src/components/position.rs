use glam::Vec3;

use crate::controller::Facing;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn move_forward(&mut self, facing: Facing) {
        let (dx, dz) = match facing {
            Facing::North => (0., -1.),
            Facing::South => (0., 1.),
            Facing::East => (1., 0.),
            Facing::West => (-1., 0.),
        };

        self.x += dx;
        self.z += dz;
    }

    pub fn move_backward(&mut self, facing: Facing) {
        let (dx, dz) = match facing {
            Facing::North => (0., 1.),
            Facing::South => (0., -1.),
            Facing::East => (-1., 0.),
            Facing::West => (1., 0.),
        };

        self.x += dx;
        self.z += dz;
    }

    pub fn move_left(&mut self, facing: Facing) {
        let (dx, dz) = match facing {
            Facing::North => (-1., 0.),
            Facing::South => (1., 0.),
            Facing::East => (0., -1.),
            Facing::West => (0., 1.),
        };

        self.x += dx;
        self.z += dz;
    }

    pub fn move_right(&mut self, facing: Facing) {
        let (dx, dz) = match facing {
            Facing::North => (1., 0.),
            Facing::South => (-1., 0.),
            Facing::East => (0., 1.),
            Facing::West => (0., -1.),
        };

        self.x += dx;
        self.z += dz;
    }
}

impl Into<Vec3> for Position {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
