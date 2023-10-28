use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use glam::Vec3;
use log::*;

use gobs_game as game;
use gobs_scene as scene;

use game::input::Key;

use scene::camera::Camera;

#[derive(Copy, Clone, Debug)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn yaw(&self) -> f32 {
        match self {
            Facing::North => -FRAC_PI_2,
            Facing::South => FRAC_PI_2,
            Facing::East => 0.,
            Facing::West => PI,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::South => Facing::West,
            Facing::East => Facing::South,
            Facing::West => Facing::North,
        }
    }
}

#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    fov_up: f32,
    fov_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    sensitivity: f32,
    debug: bool,
    mouse_pressed: bool,
    facing: Facing,
}

impl CameraController {
    pub fn new(facing: Facing, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.,
            amount_right: 0.,
            amount_forward: 0.,
            amount_backward: 0.,
            amount_up: 0.,
            amount_down: 0.,
            fov_up: 0.,
            fov_down: 0.,
            rotate_horizontal: 0.,
            rotate_vertical: 0.,
            scroll: 0.,
            sensitivity,
            debug: false,
            mouse_pressed: false,
            facing,
        }
    }

    pub fn mouse_pressed(&mut self) {
        self.mouse_pressed = true;
    }

    pub fn mouse_released(&mut self) {
        self.mouse_pressed = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.key_event(key, true);
    }

    pub fn key_released(&mut self, key: Key) {
        self.key_event(key, false);
    }

    fn key_event(&mut self, key: Key, pressed: bool) {
        let amount = if pressed { 1. } else { 0. };

        match key {
            Key::A => {
                if pressed {
                    self.facing = self.facing.turn_left();
                }
            }
            Key::E => {
                if pressed {
                    self.facing = self.facing.turn_right();
                }
            }
            Key::Z | Key::Up => {
                self.amount_forward = amount;
            }
            Key::S | Key::Down => {
                self.amount_backward = amount;
            }
            Key::Q | Key::Left => {
                self.amount_left = amount;
            }
            Key::D | Key::Right => {
                self.amount_right = amount;
            }
            Key::Space => {
                self.amount_up = amount;
            }
            Key::LShift => {
                self.amount_down = amount;
            }
            Key::L => {
                self.debug = true;
            }
            Key::PageUp => {
                self.fov_up = amount;
            }
            Key::PageDown => {
                self.fov_down = amount;
            }
            _ => (),
        }
    }

    pub fn mouse_drag(&mut self, mouse_dx: f64, mouse_dy: f64) {
        if self.mouse_pressed {
            self.rotate_horizontal = mouse_dx as f32;
            self.rotate_vertical = -mouse_dy as f32;
        }
    }

    pub fn mouse_scroll(&mut self, delta: f32) {
        self.scroll = delta;
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32) {
        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = Vec3::new(yaw_cos, 0., yaw_sin).normalize();
        let right = Vec3::new(-yaw_sin, 0., yaw_cos).normalize();
        let up = Vec3::new(0., 1., 0.);

        let mut position = camera.position;

        position += forward * (self.amount_forward - self.amount_backward);
        position += right * (self.amount_right - self.amount_left);
        position += up * (self.amount_up - self.amount_down);

        camera.position = position;

        if !self.mouse_pressed {
            camera.yaw = self.facing.yaw();
            camera.pitch = 0.;
        }

        self.amount_forward = 0.;
        self.amount_backward = 0.;
        self.amount_right = 0.;
        self.amount_left = 0.;
        self.amount_up = 0.;
        self.amount_down = 0.;

        camera.projection.fovy += (self.fov_up - self.fov_down) * dt;

        camera.yaw += self.rotate_horizontal * self.sensitivity * dt;
        camera.pitch += self.rotate_vertical * self.sensitivity * dt;

        self.rotate_horizontal = 0.;
        self.rotate_vertical = 0.;

        if self.debug {
            warn!("{}", camera);
            self.debug = false;
        }
    }
}
