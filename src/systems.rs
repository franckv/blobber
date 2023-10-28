mod camera;
mod collider;
mod input;
mod mover;

use std::sync::Arc;

use gobs_scene::{Model, scene::Scene};
use hecs::World;

use crate::{events::Event, map::TileMap};

pub fn update(
    world: &mut World,
    events: &Vec<Event>,
    map: &TileMap<Arc<Model>>,
    scene: &mut Scene,
) {
    input::input_system(world, events);
    collider::collide_system(world, map);
    mover::move_system(world);
    camera::camera_system(world, scene);
}
