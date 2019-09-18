use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, Write, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{
        Worker,
        Velocity,
        SimpleAnimation
    }
};

pub struct WorkerSystem;

impl<'s> System<'s> for WorkerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SimpleAnimation>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>
    );

    fn run(&mut self, (entities, workers, velocities, mut locals, mut animations, mut sprites, time): Self::SystemData) {
        for (entity, _, velocity, local) in (&entities, &workers, &velocities, &mut locals).join() {
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());
        }
    }
}