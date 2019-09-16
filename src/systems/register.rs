use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Destination, Emotion, Feeling, Food, Patron, Register, Velocity};

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
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Feeling>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut patrons,
            registers,
            foods,
            mut parents,
            mut locals,
            mut destinations,
            mut velocities,
            mut feelings,
            time,
        ): Self::SystemData,
    ) {
        let mut food_locals_to_reset: Vec<(Entity, Transform)> = vec![];

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

                let dist =
                    get_distance_between_two_points([register_x, register_y], [patron_x, patron_y]);
                let is_close_enough: bool = dist < 2.0;
                if is_close_enough {
                    // Determine if any unattached food entities of the right type exists.
                    // TODO

                    // If so, attach the available food item to the patron and send them off.
                    for (food_entity, food, food_local) in (&entities, &foods, &locals).join() {
                        parents
                            .insert(
                                food_entity,
                                Parent {
                                    entity: patron_entity,
                                },
                            )
                            .unwrap();
                        destinations
                            .insert(patron_entity, Destination { x: 0.0, y: 0.0 })
                            .unwrap();

                        // Arrange the food item relative to the person
                        let mut local = Transform::default();
                        local.prepend_translation_z(0.2);
                        local.prepend_translation_x(8.0);
                        local.prepend_translation_y(-8.0);
                        food_locals_to_reset.push((food_entity, local));

                        // Reset the patron's walking speed
                        let mut velocity = velocities.get(patron_entity).unwrap();
                        velocities
                            .insert(patron_entity, velocity.set_displacement(15.0))
                            .unwrap();

                        // Update the patron's thought to happiness
                        match feelings.get_mut(patron_entity) {
                            Some(feeling) => {
                                feeling.symbol = Emotion::Happy;
                                dbg!("should update");
                            },
                            None => {
                                dbg!("no feelings");
                            }
                        };
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

        for (entity, local) in food_locals_to_reset {
            locals.insert(entity, local).unwrap();
        }
    }
}
