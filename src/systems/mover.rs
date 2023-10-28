use hecs::World;

use crate::components::{Intent, Action, Direction, Orientation, Position};

pub fn move_system(world: &mut World) {
    let mut to_remove = Vec::new();

    world.query::<(&mut Orientation, &mut Position, &Intent)>().iter().for_each(|(e, (orientation, position, intent))| {
        let Intent {action } = intent;
        match action {
            Action::Move(Direction::Forward) => {
                position.move_forward(orientation.facing);
            },
            Action::Move(Direction::Backward) => {
                position.move_backward(orientation.facing);
            },
            Action::Move(Direction::Left) => {
                position.move_left(orientation.facing);
            },
            Action::Move(Direction::Right) => {
                position.move_right(orientation.facing);
            },
            Action::Turn(Direction::Left) => {
                orientation.facing = orientation.facing.turn_left();
            },
            Action::Turn(Direction::Right) => {
                orientation.facing = orientation.facing.turn_right();
            },
            _ => panic!()
        }
        to_remove.push(e);
    });

    for e in to_remove {
        world.remove::<(Intent,)>(e).unwrap();
    }
}
