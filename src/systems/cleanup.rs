use hecs::World;

use crate::components::{Animation, Intent};

pub fn cleanup_system(world: &mut World) {
    cleanup_intents(world);
}

pub fn cleanup_intents(world: &World) {
    world
        .query::<(&Intent,)>()
        .without::<&Animation>()
        .iter()
        .for_each(|(_, intent)| {
            panic!(">>> Lingering intent {:?}", intent);
        });
}
