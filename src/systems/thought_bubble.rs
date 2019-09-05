use amethyst::{
    core::timing::Time,
    core::transform::{
        Transform,
        Parent
    },
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Patron,
    ThoughtBubble
};

pub struct MoveThoughtBubbleSystem;

impl<'s> System<'s> for MoveThoughtBubbleSystem {
    type SystemData = (
        ReadStorage<'s, Patron>,
        WriteStorage<'s, ThoughtBubble>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (patrons, thought_bubbles, parents, mut locals, time): Self::SystemData) {}
}