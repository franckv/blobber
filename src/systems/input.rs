use hecs::{CommandBuffer, World};

use gobs::game::input::{Input, Key};

use crate::{
    components::{Action, Intent, Player},
    events::Event,
    movement::Direction,
};

pub fn input_system(world: &mut World, events: &Vec<Event>, delta: f32) {
    let mut action = Action::None;

    let mut stop = false;

    events.iter().for_each(|e| {
        if !stop {
            if let Event::Input(Input::KeyPressed(key)) = e {
                action = match key {
                    Key::A => Action::Turn(Direction::Left),
                    Key::E => Action::Turn(Direction::Right),
                    Key::Z => Action::Move(Direction::Forward),
                    Key::Q => Action::Move(Direction::Left),
                    Key::D => Action::Move(Direction::Right),
                    Key::S => Action::Move(Direction::Backward),
                    _ => Action::None,
                }
            }

            if let Event::Input(Input::MouseMotion(dx, dy)) = e {
                action = Action::Look((*dx as f32 * delta, *dy as f32 * delta));
            }

            if let Event::Input(Input::MouseReleased) = e {
                action = Action::ControlCamera(false);
                stop = true;
            } else if let Event::Input(Input::MousePressed) = e {
                action = Action::ControlCamera(true);
                stop = true;
            }
        }
    });

    if action != Action::None {
        let mut cmd = CommandBuffer::new();

        world
            .query_mut::<&Player>()
            .without::<&Intent>()
            .into_iter()
            .for_each(|(e, _)| {
                cmd.insert(e, (Intent { action },));
            });

        cmd.run_on(world);
    }
}
