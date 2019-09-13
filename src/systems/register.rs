use amethyst::{
    core::timing::Time,
    core::transform::{Transform, Parent},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Destination, Patron, Register, Food}
};

pub struct RegisterSystem;

// Put this in a lib!
fn get_distance_between_two_points(point_a: [f32; 2], point_b: [f32; 2]) -> f32 {
    let x = point_a[0] - point_b[0];
    let y = point_a[1] - point_b[1];
    let c: f32 = x.powi(2) + y.powi(2);
    c.sqrt()
}

impl<'s> System<'s> for RegisterSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Register>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, Parent>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut patrons, registers, foods, mut parents, locals, mut destinations, time): Self::SystemData,
    ) {
        // For each register
        for (register, register_local) in (&registers, &locals).join() {
            // Check to see if a patron is in range by looking at x value of register...
            let register_translation = register_local.translation();
            let register_x = register_translation.x;
            let register_y = register_translation.y;

            for (patron_entity, patron, patron_local) in (&*entities, &mut patrons, &locals).join()
            {
                // and comparing it to the x of each patron
                let patron_translation = patron_local.translation();
                let patron_x = patron_translation.x;
                let patron_y = patron_translation.y;

                let dist = get_distance_between_two_points([register_x, register_y],[patron_x, patron_y]);
                let is_close_enough : bool = dist < 2.0;
                if is_close_enough {
                    // Determine if any unattached food entities of the right type exists.

                    // If so, attach a Parent component to it, passing the patron entity.
                    for (food_entity, food) in (&entities, &foods).join() {
                        parents.insert(food_entity, Parent {
                            entity: patron_entity
                        }).unwrap();
                        destinations
                            .insert(
                                patron_entity,
                                Destination {
                                    x: 0.0,
                                    y: 0.0
                                }
                            ).unwrap();
                    }

                } else if register_x.floor() == patron_x.floor() {
                    // If there's a match, attract the patron
                    // Updating the Patron's destination will cause it to walk
                    // towards the register
                    destinations
                        .insert(
                            patron_entity,
                            Destination {
                                x: register_x,
                                y: register_y,
                            },
                        )
                        .unwrap();
                }
            }
        }
    }
}
