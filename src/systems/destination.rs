use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Destination, Patron, Velocity};

pub struct DestinationSystem;

fn get_distance_between_two_points(point_a: [f32; 2], point_b: [f32; 2]) -> f32 {
    let x = point_a[0] - point_b[0];
    let y = point_a[1] - point_b[1];
    let c: f32 = x.powi(2) + y.powi(2);
    c.sqrt()
}

impl<'s> System<'s> for DestinationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Patron>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
    );

    fn run(
        &mut self,
        (entities, mut patrons, mut velocities, mut locals, mut destinations): Self::SystemData,
    ) {
        let mut velocities_to_insert: Vec<(Entity, Velocity)> = vec![];
        let mut destinations_to_insert: Vec<(Entity, Destination)> = vec![];

        for (entity, patron, velocity, patron_local, dest) in
            (&entities, &patrons, &mut velocities, &mut locals, &mut destinations).join()
        {
            dbg!(dest.clone());

            // Did patron arrive at their destination?
            let pos = patron_local.translation();
            let dist = get_distance_between_two_points([pos.x, pos.y], [dest.x, dest.y]);
            let is_getting_close: bool = dist < 8.0;
            let is_close_enough: bool = dist < 4.0;
            if (is_close_enough) {
                dbg!("close enough!");
                // If so,
                // - remove the destination
                // - zero out velocity
                // - snap the patron to the destination
                patron_local.set_translation_xyz(dest.x, dest.y, pos.z);
                destinations_to_insert.push((entity, Destination { x: 0.0, y: 0.0 }));
                // velocities_to_insert.push((entity, Velocity { x: 0.0, y: 0.0 }));
            } else {
                dbg!("not close enough!");
                // If getting close, start to slow down.
                if (is_getting_close) {
                    let mut displacement = velocity.get_displacement() * 0.99;
                    if displacement < 4.0 {
                        displacement = 4.0;
                    }

                    *velocity = velocity.set_displacement(displacement);
                }

                // If not, prepare to change velocity
                velocities_to_insert.push((
                    entity,
                    velocity.turn(
                        Destination { x: pos.x, y: pos.y },
                        Destination {
                            x: dest.x,
                            y: dest.y,
                        },
                    ),
                ));
            }
        }

        // Update velocities
        for (entity, velocity) in velocities_to_insert {
            velocities.insert(entity, velocity).unwrap();
        }

        // New Destination test
        for (entity, destination) in destinations_to_insert {
            destinations.insert(entity, destination).unwrap();
            dbg!("should have inserted");
            dbg!(entity);
            dbg!(destination);
        }
    }
}
