use amethyst::{
    core::timing::Time,
    core::transform::{
        Transform,
        Parent
    },
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Food,
    ThoughtBubble
};

pub struct MoveFoodSystem;

impl<'s> System<'s> for MoveFoodSystem {
    type SystemData = (
        ReadStorage<'s, ThoughtBubble>,
        WriteStorage<'s, Food>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (thought_bubbles, foods, parents, mut locals, time): Self::SystemData) {}
}