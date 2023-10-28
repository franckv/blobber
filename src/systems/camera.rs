use gobs_scene::scene::Scene;
use hecs::World;

use crate::components::{Camera, Orientation, Position};

pub fn camera_system(world: &mut World, scene: &mut Scene) {
    let camera = world
        .query_mut::<(&Camera, &Position, &Orientation)>()
        .into_iter()
        .map(|(_, (_, position, orientation))| (position, orientation))
        .next();

    match camera {
        Some((position, orientation)) => {
            scene.camera.position = (*position).into();
            scene.camera.yaw = orientation.facing.yaw();
        }
        None => panic!(),
    }
}
