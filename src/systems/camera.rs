use gobs::scene::Scene;
use hecs::{CommandBuffer, World};

use crate::components::{Camera, Intent, Orientation, Position};

pub fn camera_system(world: &mut World, scene: &mut Scene) {
    move_camera(world);
    update_scene(world, scene);
}

pub fn move_camera(world: &mut World) {
    let mut cmd = CommandBuffer::new();

    world
        .query_mut::<(&mut Camera, &Intent)>()
        .into_iter()
        .for_each(|(e, (camera, intent))| {
            let Intent { action } = intent;

            match action {
                crate::components::Action::Look((dx, dy)) => {
                    if camera.free_view {
                        camera.yaw += dx;
                        camera.pitch -= dy;
                    } else {
                        camera.yaw = 0.;
                        camera.pitch = 0.;
                    }
                    cmd.remove::<(Intent,)>(e);
                }
                crate::components::Action::ControlCamera(lock) => {
                    camera.free_view = *lock;
                    if !lock {
                        camera.yaw = 0.;
                        camera.pitch = 0.;
                    }
                    cmd.remove::<(Intent,)>(e);
                }
                _ => (),
            }
        });

    cmd.run_on(world);
}

pub fn update_scene(world: &World, scene: &mut Scene) {
    world
        .query::<(&Camera, &Position, &Orientation)>()
        .iter()
        .for_each(|(_, (camera, position, orientation))| {
            scene.camera.position = (*position).into();
            scene.camera.yaw = orientation.yaw + camera.yaw;
            scene.camera.pitch = camera.pitch;
        });
}
