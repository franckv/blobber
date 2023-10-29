use hecs::{CommandBuffer, World};

use crate::{
    components::{Action, Animation, Intent, Orientation, Position},
    movement,
};

pub fn move_system(world: &mut World) {
    let mut cmd = instant_move(world);
    cmd.run_on(world);

    animated_move(world);
}

fn instant_move(world: &mut World) -> CommandBuffer {
    let mut cmd = CommandBuffer::new();

    world
        .query_mut::<(&mut Orientation, &mut Position, &Intent)>()
        .without::<&Animation>()
        .into_iter()
        .for_each(|(e, (orientation, position, intent))| {
            let Intent { action } = intent;
            match action {
                Action::Move(direction) => {
                    let translation = movement::get_translation(orientation.facing, *direction, 1.);
                    position.translate(translation, true);
                }
                Action::Turn(direction) => {
                    orientation.rotate(*direction, 1., true);
                }
                _ => panic!(),
            }
            cmd.remove::<(Intent,)>(e);
        });

    cmd
}

fn animated_move(world: &mut World) {
    world
        .query_mut::<(&mut Orientation, &mut Position, &Intent, &Animation)>()
        .into_iter()
        .for_each(|(_, (orientation, position, intent, animation))| {
            let Intent { action } = intent;
            match action {
                Action::Move(direction) => {
                    let translation = movement::get_translation(
                        orientation.facing,
                        *direction,
                        1. / animation.frames as f32,
                    );
                    position.translate(translation, animation.last());
                }
                Action::Turn(direction) => {
                    orientation.rotate(*direction, 1. / animation.frames as f32, animation.last());
                }
                _ => panic!(),
            }
        });
}
