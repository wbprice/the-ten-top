use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Patron,
    Parent,
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

    fn run(&mut self, (patrons, thought_bubbles, parents, mut locals, time): Self::SystemData) {
        for (thought_bubble, parent, local) in (&thought_bubbles, &parents, &mut locals).join() {
            let thought_bubble_x = local.translation().x;
            let patron = patrons.get(parent.entity).unwrap();

            local.prepend_translation_x(patron.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(patron.velocity[1] * time.delta_seconds());

            // While waiting for more clever behavior to be implemented
            // Run off the screen but reset position
            let bubble_x = local.translation().x;
            if bubble_x > 160.0 {
                local.set_translation_x(0.0);
            }
        }
    }
}