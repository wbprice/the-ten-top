use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Patron,
    Destination
};

pub struct DestinationSystem;

impl<'s> System<'s> for DestinationSystem {
    type SystemData = (
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Destination>
    );

    fn run(&mut self, (mut patrons, locals, destinations): Self::SystemData) {
        for (patron) in (&patrons, &locals, &destinations).join() {
            dbg!(patron);
        }
    }
}