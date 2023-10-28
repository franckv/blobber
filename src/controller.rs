use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use log::*;

use game::input::Key;
use gobs_game as game;
use gobs_scene as scene;
use scene::camera::Camera;

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

    pub fn turn_left(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South,
        }
    }

    pub fn turn_right(self) -> Self {
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
    fov_up: f32,
    fov_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    sensitivity: f32,
    debug: bool,
    mouse_pressed: bool,
}

impl CameraController {
    pub fn new(sensitivity: f32) -> Self {
        Self {
            fov_up: 0.,
            fov_down: 0.,
            rotate_horizontal: 0.,
            rotate_vertical: 0.,
            scroll: 0.,
            sensitivity,
            debug: false,
            mouse_pressed: false,
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
        if !self.mouse_pressed {
            camera.pitch = 0.;
        }

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
