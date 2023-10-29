mod animate;
mod camera;
mod cleanup;
mod collider;
mod input;
mod mover;

use std::sync::Arc;

use gobs_scene::{scene::Scene, Model};
use hecs::World;

use crate::{events::Event, map::TileMap};

pub fn update(
    delta: f32,
    world: &mut World,
    events: &Vec<Event>,
    map: &TileMap<Arc<Model>>,
    scene: &mut Scene,
) {
    input::input_system(world, events, delta);
    collider::collide_system(world, map);
    animate::animate_system(world);
    mover::move_system(world);
    camera::camera_system(world, scene);
    cleanup::cleanup_system(world);
}
