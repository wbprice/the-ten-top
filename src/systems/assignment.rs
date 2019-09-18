use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, Write, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{
        Worker,
        Assignment,
        Patron,
        Destination,
        Velocity
    },
    resources::{
        GameState,
        Task
    }
};

pub struct AssignmentSystem;

impl<'s> System<'s> for AssignmentSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        ReadStorage<'s, Patron>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Assignment>,
        WriteStorage<'s, Velocity>,
        Write<'s, GameState>
    );

    fn run(&mut self, (entities, workers, patrons, locals, mut destinations, mut assignments, mut velocities, mut game_state): Self::SystemData) {
        // If a new task needs to be performed
        if let Some(task) = game_state.tasks.pop() {
            // Find an idle worker and assign them the task.
            match (&entities, &workers, !&assignments).join().next() {
                Some((worker_entity, _, _)) => {
                    // Note that worker is busy with this task
                    assignments.insert(worker_entity, Assignment { task: task.clone() }).unwrap();

                    // Perform one-time updates to state to get the workers working.
                    match &task {
                        Task::TakeOrder { patron } => {
                            // Where is the patron in question?
                            let patron_local = locals.get(*patron).unwrap();
                            let patron_translation = patron_local.translation();

                            // Add a destination component to the worker.
                            // about one cell above the patron.
                            destinations.insert(
                                worker_entity,
                                Destination {
                                    x: patron_translation.x,
                                    y: patron_translation.y + 24.0
                                }
                            ).unwrap();

                            velocities.insert(
                                worker_entity,
                                Velocity {
                                    x: 15.0,
                                    y: 0.0
                                }
                            ).unwrap();
                        },
                        _ => {}
                    }
                },
                None => {}
            }
        }
    }
}