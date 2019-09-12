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
    let c : f32 = x.powi(2) + y.powi(2);
    c.sqrt()
}

impl<'s> System<'s> for DestinationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Patron>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
    );

    fn run(
        &mut self,
        (entities, mut patrons, mut velocities, locals, mut destinations): Self::SystemData,
    ) {
        let mut destinations_to_remove: Vec<Entity> = vec![];
        let mut velocities_to_insert: Vec<(Entity, Velocity)> = vec![];

        for (entity, patron, velocity, patron_local, destination) in
            (&entities, &patrons, &mut velocities, &locals, &destinations).join()
        {
            // Did patron arrive at their destination?
            let pos = patron_local.translation();

            let dist = get_distance_between_two_points([pos.x, pos.y], [destination.x, destination.y]);
            let close_enough : bool = dist < 2.0;
            if close_enough {
                // If so, remove the destination and zero out velocity.
                destinations_to_remove.push(entity);
                velocities_to_insert.push((entity, Velocity { x: 0.0, y: 0.0 }));
            } else {

                // If getting close, start to slow down.
                if ((pos.x - destination.x).abs() < 8.0 &&
                    (pos.y - destination.y).abs() < 8.0) {

                    let mut new_displacement = velocity.get_displacement() * 0.99;
                    if new_displacement < 8.0 {
                        new_displacement = 8.0;
                    }

                    *velocity = velocity.set_displacement(new_displacement);
                }
                
                // If not, change their velocity
                let new_velocity = velocity.turn(
                    Destination { x: pos.x, y: pos.y },
                    Destination {
                        x: destination.x,
                        y: destination.y,
                    },
                );

                let new_velocity_entity = (
                    entity,
                    new_velocity
                );

                velocities_to_insert.push(new_velocity_entity);
            }
        }

        // Clean up entities
        for entity in destinations_to_remove {
            destinations.remove(entity).unwrap();
        }

        // Update velocities
        for (entity, velocity) in velocities_to_insert {
            velocities.insert(entity, velocity).unwrap();
        }
    }
}
