use crate::movement::Direction;

use super::{Orientation, Position};

#[derive(Clone, Copy, Debug)]
pub enum AnimationType {
    TRANSLATE(Position, Direction),
    ROTATE(Orientation, Direction),
}

#[derive(Clone, Copy, Debug)]
pub struct Animation {
    pub effect: AnimationType,
    pub progress: u32,
    pub frames: u32,
}

impl Animation {
    pub fn new(effect: AnimationType) -> Self {
        Animation {
            effect,
            progress: 0,
            frames: 30,
        }
    }

    pub fn progress(&mut self) -> bool {
        self.progress += 1;

        return self.progress >= self.frames;
    }

    pub fn last(&self) -> bool {
        self.progress == self.frames - 1
    }
}
