use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{
        Worker
    },
    resources::{
        GameState
    }
};

pub struct MoveWorkerSystem;

impl<'s> System<'s> for MoveWorkerSystem {
    type SystemData = (
        ReadStorage<'s, Worker>,
        Read<'s, GameState>
    );

    fn run(&mut self, (workers, game_state): Self::SystemData) {
        for (worker) in (&workers).join() {
            
        }
    }
}