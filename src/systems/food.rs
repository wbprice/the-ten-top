use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Food};

pub struct FoodSystem;

impl<'s> System<'s> for FoodSystem {
    type SystemData = (
        ReadStorage<'s, Food>
    );

    fn run(&mut self, (Food): Self::SystemData) {}
}
