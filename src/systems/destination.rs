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

        for (entity, patron, patron_local, destination) in
            (&entities, &patrons, &locals, &destinations).join()
        {
            // Did patron arrive at their destination?
            let pos = patron_local.translation();
            if pos.x.floor() == destination.x.floor() && pos.y.floor() == destination.y.floor() {
                dbg!("arrived");
                // If so, remove the destination and zero out velocity.
                destinations_to_remove.push(entity.clone());
                velocities
                    .insert(entity, Velocity { x: 0.0, y: 0.0 })
                    .unwrap();
            } else {
                dbg!(patron);
                dbg!()
                // If not, are they going the right way?

                // If not, change their velocity
            }
        }

        // Clean up entities
        for entity in destinations_to_remove {
            destinations.remove(entity);
        }
    }
}
