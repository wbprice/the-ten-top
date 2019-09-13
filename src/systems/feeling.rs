use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Feeling, ThoughtBubble};

pub struct MoveFeelingSystem;

impl<'s> System<'s> for MoveFeelingSystem {
    type SystemData = (
        ReadStorage<'s, Feeling>
    );

    fn run(&mut self, (feelings): Self::SystemData) {}
}
