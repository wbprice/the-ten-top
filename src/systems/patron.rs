use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Patron;

pub struct MovePatronSystem;

impl<'s> System<'s> for MovePatronSystem {
    type SystemData = (
        ReadStorage<'s, Patron>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (patrons, mut locals, time): Self::SystemData) {
        for (patron, local) in (&patrons, &mut locals).join() {
            local.prepend_translation_x(patron.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(patron.velocity[1] * time.delta_seconds());

            let patron_x = local.translation().x;
            if patron_x > 160.0 {
                local.set_translation_x(0.0);
            }

            dbg!(patron_x);
        }
    }
}
