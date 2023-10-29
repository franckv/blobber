use std::f32::consts::FRAC_PI_2;

use crate::movement::{Direction, Facing};

#[derive(Clone, Copy, Debug)]
pub struct Orientation {
    pub facing: Facing,
    pub yaw: f32,
}

impl Orientation {
    pub fn new(facing: Facing) -> Self {
        Orientation {
            facing,
            yaw: facing.yaw(),
        }
    }

    pub fn face(&mut self, facing: Facing) {
        self.facing = facing;
        self.yaw = facing.yaw()
    }

    pub fn rotate(&mut self, direction: Direction, amount: f32, clip: bool) {
        let amount = amount * FRAC_PI_2;
        if clip {
            self.facing = self.facing.turn(direction);
            self.yaw = self.facing.yaw();
        } else {
            match direction {
                Direction::Left => self.yaw -= amount,
                Direction::Right => self.yaw += amount,
                Direction::Forward => (),
                Direction::Backward => self.yaw += amount,
            }
        }
    }
}
