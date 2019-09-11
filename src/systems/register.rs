use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, Entities, WriteStorage},
};

use crate::components::{
    Patron,
    Register,
    Destination
};

pub struct RegisterSystem;

impl<'s> System<'s> for RegisterSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Register>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
        Read<'s, Time>
    );

    fn run(&mut self, (entities, mut patrons, registers, locals, mut destinations, time): Self::SystemData) {
        // For each register
        for (register, register_local) in (&registers, &locals).join() {
            // Check to see if a patron is in range by looking at x value of register...
            let register_x = register_local.translation().x;

            for (patron_entity, patron, patron_local) in (&*entities, &mut patrons, &locals).join() {
                // and comparing it to the x of each patron
                let patron_x = patron_local.translation().x;

                // If there's a match, attract the patron
                if register_x.floor() == patron_x.floor() {

                    // Updating the Patron's destination will cause it to walk
                    // towards the register
                    destinations.insert(patron_entity, Destination {
                        destination: [0.0, 0.0]
                    }).unwrap();
                }
            }
        }
    }
}