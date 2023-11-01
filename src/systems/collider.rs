use std::sync::Arc;

use glam::Vec3;
use hecs::{CommandBuffer, World};
use log::error;

use gobs::scene::Model;

use crate::{
    components::{Action, Animation, Intent, Orientation, Position},
    map::TileMap,
    movement,
};

pub fn collide_system(world: &mut World, map: &TileMap<Arc<Model>>) {
    let mut cmd = CommandBuffer::new();

    world
        .query::<(&Orientation, &Position, &Intent)>()
        .without::<(&Animation,)>()
        .iter()
        .for_each(|(e, (orientation, position, intent))| {
            let Intent { action } = intent;
            let mut translation = Vec3::ZERO;

            match action {
                Action::Move(direction) => {
                    translation = movement::get_translation(orientation.facing, *direction, 1.);
                }
                _ => (),
            }

            let new_position = Into::<Vec3>::into(*position) + translation;
            if map.collides(new_position) {
                error!("Collide");
                cmd.remove::<(Intent,)>(e);
            }
        });

    cmd.run_on(world);
}
