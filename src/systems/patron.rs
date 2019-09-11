use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Patron, Velocity};

pub struct MovePatronSystem;

impl<'s> System<'s> for MovePatronSystem {
    type SystemData = (
        ReadStorage<'s, Patron>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (patrons, velocities, mut locals, time): Self::SystemData) {
        for (patron, velocity, local) in (&patrons, &velocities, &mut locals).join() {
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());

            // Allow patrons to run off the screen
            // If they do so, update their position to the opposite edge of the screen.
            let patron_x = local.translation().x;
            if patron_x > 200.0 {
                local.set_translation_x(0.0);
            }

            if patron_x < -40.0 {
                local.set_translation_x(160.0);
            }
        }
    }
}
