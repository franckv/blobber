use std::sync::Arc;

use gobs_scene::Model;
use hecs::World;

use crate::{components::{Intent, Action, Direction, Orientation, Position}, map::TileMap};

pub fn collide_system(world: &mut World, map: &TileMap<Arc<Model>>) {
    let mut to_remove = Vec::new();

    world.query::<(&Orientation, &Position, &Intent)>().iter().for_each(|(e, (orientation, position, intent))| {
        let Intent {action } = intent;
        let mut new_position = *position;

        match action {
            Action::Move(Direction::Forward) => {
                new_position.move_forward(orientation.facing);
            },
            Action::Move(Direction::Backward) => {
                new_position.move_backward(orientation.facing);
            },
            Action::Move(Direction::Left) => {
                new_position.move_left(orientation.facing);
            },
            Action::Move(Direction::Right) => {
                new_position.move_right(orientation.facing);
            },
            _ => ()
        }
        if map.collides(new_position.into()) {
            to_remove.push(e);
        }
    });

    for e in to_remove {
        world.remove::<(Intent,)>(e).unwrap();
    }
}
