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

impl<'s> System<'s> for MoveFoodSystem {
    type SystemData = (
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Destination>
    );

    fn run(&mut self, (mut patrons, destinations): Self::SystemData) {}
}