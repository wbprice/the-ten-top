use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::Dishes;

#[derive(Debug, Copy, Clone)]
pub struct Dish {
    pub dish: Dishes,
}

impl Component for Dish {
    type Storage = DenseVecStorage<Self>;
}
