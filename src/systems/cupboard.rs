use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{
        Cupboard,
        Ingredient
    }
};

pub struct CupboardSystem;

impl<'s> System<'s> for CupboardSystem {
    type SystemData = (
        WriteStorage<'s, CupboardSystem>,
    )
}