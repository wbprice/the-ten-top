use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, Write, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{
        Worker,
        Assignment
    },
    resources::{
        GameState
    }
};

pub struct AssignmentSystem;

impl<'s> System<'s> for AssignmentSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        WriteStorage<'s, Assignment>,
        Write<'s, GameState>
    );

    fn run(&mut self, (entities, workers, assignments, mut game_state): Self::SystemData) {
        // If a new task needs to be performed
        if let Some(task) = game_state.tasks.pop() {
            // Find an idle worker and assign them the task.
            match (&entities, &workers, !&assignments).join().next() {
                Some((worker_entity, _, _)) => {
                    assignments.insert(worker_entity, Assignment { task });
                },
                None => {}
            }
        }
    }
}