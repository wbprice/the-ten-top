use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Assignment, Destination, Patron, Velocity, Worker},
    resources::{GameState, Task, Status},
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
        // If a new task needs to be performed
        if let Some(task) = game_state.tasks.pop() {
            // Find an idle worker and assign them the task.
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

        for (worker_entity, _, assignment) in (&entities, &workers, &mut assignments).join() {
            // For workers with assignments, is the work completed?
            match assignment.task {
                Task::MoveToEntity { entity } => {
                    match &assignment.status {
                        Status::New => {
                            // Perform one-time setup of task.
                            let entity_local = locals.get(entity).unwrap();
                            let entity_transform = entity_local.translation();

                            destinations.insert(worker_entity, Destination {
                                x: entity_transform.x,
                                y: entity_transform.y
                            }).unwrap();

                            assignment.status = Status::InProgress;
                        },
                        Status::InProgress => {
                            // Check to see if task has been completed.
                            // If so, set the status to completed.
                            match destinations.get(worker_entity) {
                                Some(_) => {},
                                None => {
                                    assignment.status = Status::Completed
                                }
                            }
                        },
                        Status::Completed => {
                            // Perform any cleanup
                            // Queue up the next task?
                            // Remove the assignment?
                            dbg!("destination completed");
                        }
                    }

                }
                _ => {}
            }

        }
    }
}
