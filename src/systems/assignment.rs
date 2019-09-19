use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Assignment, Destination, Patron, Velocity, Worker},
    resources::{GameState, Status, Task},
};

pub struct AssignmentSystem;

impl<'s> System<'s> for AssignmentSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Assignment>,
        Write<'s, GameState>,
    );

    fn run(
        &mut self,
        (
            entities,
            workers,
            locals,
            mut destinations,
            mut assignments,
            mut game_state,
        ): Self::SystemData,
    ) {
        let mut assignments_to_remove: Vec<Entity> = vec![];

        // If a new task needs to be performed
        // Find an idle worker and assign them the task.
        if let Some(task) = game_state.tasks.pop() {
            match (&entities, &workers, !&assignments).join().next() {
                Some((worker_entity, _, _)) => {
                    // Note that worker is busy with this task
                    assignments
                        .insert(worker_entity, Assignment::new(task))
                        .unwrap();
                }
                None => {}
            }
        }

        // For workers with assignments:
        // Perform initial setup
        // Check to see if task was completed
        // Figure out what the next step is
        for (worker_entity, _, assignment) in (&entities, &workers, &mut assignments).join() {
            match assignment.task {
                Task::MoveToEntity { entity } => {
                    match &assignment.status {
                        Status::New => {
                            // Where is the entity to walk to?
                            let entity_local = locals.get(entity).unwrap();
                            let entity_transform = entity_local.translation();

                            // Direct the worker to walk to the entity
                            destinations
                                .insert(
                                    worker_entity,
                                    Destination {
                                        x: entity_transform.x,
                                        y: entity_transform.y,
                                    },
                                )
                                .unwrap();

                            assignment.status = Status::InProgress;
                        }
                        Status::InProgress => {
                            // Destination storage will remove the destination
                            // once the entity has reached it's destination.
                            // So the destination no longer exists, we can call
                            // the task done.
                            if let None = destinations.get(worker_entity) {
                                assignment.status = Status::Completed
                            }
                        }
                        Status::Completed => {
                            // Perform any cleanup
                            // Queue up the next task?
                            dbg!("destination completed");

                            // Remove the assignment
                            assignments_to_remove.push(worker_entity);
                        }
                    }
                }
                _ => {}
            }
        }

        // Cleanup any completed assignments, marking the worker available for the next task.
        for worker_entity in assignments_to_remove {
            match assignments.remove(worker_entity) {
                Some(thing) => {
                    dbg!(thing);
                    dbg!("removed assignment");
                }
                None => {
                    dbg!("nothing to removed?");
                }
            }
        }
    }
}
