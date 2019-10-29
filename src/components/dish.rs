use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::{Food};

#[derive(Debug, Copy, Clone)]
pub struct Dish {
    pub dish: Food::Dishes
}

impl Component for Dish {
    type Storage = DenseVecStorage<Self>;
}
