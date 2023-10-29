use gobs_game::input::{Input, Key};
use hecs::World;
use log::error;

use crate::{
    components::{Action, Intent, Player},
    events::Event,
    movement::Direction,
};

pub fn input_system(world: &mut World, events: &Vec<Event>) {
    let mut action = Action::None;

    events.iter().for_each(|e| {
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
    });

    if action != Action::None {
        error!("action={:?}", action);
        let player = world
            .query_mut::<&Player>()
            .without::<&Intent>()
            .into_iter()
            .map(|(e, _)| e)
            .next();

        match player {
            Some(e) => world.insert(e, (Intent { action },)).unwrap(),
            _ => (),
        }
    }
}
