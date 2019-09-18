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
    }
}