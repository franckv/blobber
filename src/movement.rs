use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    pub fn yaw(&self) -> f32 {
        match self {
            Facing::North => -FRAC_PI_2,
            Facing::South => FRAC_PI_2,
            Facing::East => 0.,
            Facing::West => PI,
        }
    }

    pub fn turn(self, direction: Direction) -> Self {
        match self {
            Facing::North => match direction {
                Direction::Left => Facing::West,
                Direction::Right => Facing::East,
                Direction::Forward => Facing::North,
                Direction::Backward => Facing::South,
            },
            Facing::South => match direction {
                Direction::Left => Facing::East,
                Direction::Right => Facing::West,
                Direction::Forward => Facing::South,
                Direction::Backward => Facing::North,
            },
            Facing::East => match direction {
                Direction::Left => Facing::North,
                Direction::Right => Facing::South,
                Direction::Forward => Facing::East,
                Direction::Backward => Facing::West,
            },
            Facing::West => match direction {
                Direction::Left => Facing::South,
                Direction::Right => Facing::North,
                Direction::Forward => Facing::West,
                Direction::Backward => Facing::East,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub fn get_translation(facing: Facing, direction: Direction, amount: f32) -> Vec3 {
    let move_direction = facing.turn(direction);

    match move_direction {
        Facing::North => Vec3::new(0., 0., -amount),
        Facing::South => Vec3::new(0., 0., amount),
        Facing::East => Vec3::new(amount, 0., 0.),
        Facing::West => Vec3::new(-amount, 0., 0.),
    }
}
