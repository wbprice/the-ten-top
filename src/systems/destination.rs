use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Destination, Patron, Velocity};

pub struct DestinationSystem;

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
            if pos.x == destination.x && pos.y == destination.y {
                // If so, remove the destination and zero out velocity.
                destinations_to_remove.push(entity.clone());
                velocities_to_insert.push((entity, Velocity { x: 0.0, y: 0.0 }));
            } else {
                // If not, change their velocity
                velocities_to_insert.push((
                    entity,
                    velocity.turn(
                        Destination { x: pos.x, y: pos.y },
                        Destination {
                            x: destination.x,
                            y: destination.y,
                        },
                    ),
                ));
            }
        }

        // Clean up entities
        for entity in destinations_to_remove {
            destinations.remove(entity);
        }

        // Update velocities
        for (entity, velocity) in velocities_to_insert {
            velocities.insert(entity, velocity).unwrap();
        }
    }
}
