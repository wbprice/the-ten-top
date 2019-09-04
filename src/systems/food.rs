use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Food
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

    fn run(&mut self, (thought_bubbles, foods, parents, mut locals, time): Self::SystemData) {
        for (food, parent, local) in (&foods, &parents, &mut locals).join() {
            let food_x = local.translation().x;
            let thought_bubble = thought_bubbles.get(parent.entity).unwrap();

            local.prepend_translation_x(thought_bubble.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(patron.velocity[1] * time.delta_seconds());

            // While waiting for more clever behavior to be implemented
            // Run off the screen but reset position
            let food_x = local.translation().x;
            if food_x > 160.0 {
                local.set_translation_x(0.0);
            }
        }
    }
}