use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, Read, System, WriteStorage},
    renderer::SpriteRender
};

use crate::{
    components::{
        Plate
    }
};

pub struct PlateSystem;

impl<'s> System<'s> for PlateSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Plate>
    );

    fn run(&mut self, (entities, plates): Self::SystemData) {
        // noop
    }
}