use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Destination, Patron, Velocity, Worker};

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
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
    );

    fn run(&mut self, (entities, mut velocities, locals, mut destinations): Self::SystemData) {
        let mut velocities_to_insert: Vec<(Entity, Velocity)> = vec![];
        let mut destinations_to_remove: Vec<Entity> = vec![];

        for (entity, velocity, local, dest) in
            (&entities, &mut velocities, &locals, &mut destinations).join()
        {
            dbg!("starting destination");
            // If velocity is zeroed out, bump it to walking speed.
            if velocity.get_displacement() == 0.0 {
                velocity.x = 15.0;
            }

            // Did entity arrive at their destination?
            let pos = local.translation();
            let dist = get_distance_between_two_points([pos.x, pos.y], [dest.x, dest.y]);
            let is_getting_close: bool = dist < 4.0;
            let is_close_enough: bool = dist < 2.0;

            if is_close_enough {
                // If so,
                // - remove the destination
                // - zero out velocity
                destinations_to_remove.push(entity);
                velocities_to_insert.push((entity, Velocity { x: 0.0, y: 0.0 }));
            } else {
                // If getting close, start to slow down.
                if is_getting_close {
                    let mut displacement = velocity.get_displacement() * 0.90;
                    if displacement < 4.0 {
                        displacement = 4.0;
                    }
                    *velocity = velocity.set_displacement(displacement);
                }

                // If not, change velocity so the entity is heading the right direction
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

        // Remove destinations
        for entity in destinations_to_remove {
            destinations.remove(entity);
        }
    }
}
