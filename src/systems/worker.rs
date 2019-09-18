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
        Task
    }
};

pub struct WorkerSystem;

impl<'s> System<'s> for WorkerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        ReadStorage<'s, Assignment>
    );

    fn run(&mut self, (entities, workers, assignments): Self::SystemData) {
        // Drive behavior of workers with assignments
        for (worker_entity, worker, assignment) in (&entities, &workers, &assignments).join() {

            match assignment.task {
                Task::TakeOrder => {},
                Task::FetchIngredient => {},
                Task::PlateOrder => {},
                Task::PrepIngredient = {},
                Task::DeliverOrder = {},
                _ => {}
            }

        }
    }
}