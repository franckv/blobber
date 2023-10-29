use hecs::{CommandBuffer, World};
use log::debug;

use crate::components::{Action, Animation, AnimationType, Intent, Orientation, Position};

pub fn animate_system(world: &mut World) {
    let mut cmd = update_animation(world);
    cmd.run_on(world);
    cmd = add_animation(world);
    cmd.run_on(world);
}

pub fn add_animation(world: &mut World) -> CommandBuffer {
    let mut cmd = CommandBuffer::new();

    world
        .query::<(&Orientation, &Position, &Intent)>()
        .without::<&Animation>()
        .iter()
        .for_each(|(e, (orientation, position, intent))| {
            let Intent { action } = intent;
            match action {
                Action::Move(direction) => {
                    cmd.insert(
                        e,
                        (Animation::new(AnimationType::TRANSLATE(
                            *position, *direction,
                        )),),
                    );
                }
                Action::Turn(direction) => {
                    cmd.insert(
                        e,
                        (Animation::new(AnimationType::ROTATE(
                            *orientation,
                            *direction,
                        )),),
                    );
                }
                _ => (),
            }
        });

    cmd
}

fn update_animation(world: &mut World) -> CommandBuffer {
    let mut cmd = CommandBuffer::new();

    world
        .query_mut::<(&mut Animation,)>()
        .into_iter()
        .for_each(|(e, (animation,))| {
            let finished = animation.progress();

            if finished {
                debug!("Animation done");
                cmd.remove::<(Animation,)>(e);
                cmd.remove::<(Intent,)>(e);
            }
        });

    cmd
}
