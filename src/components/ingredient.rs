use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::{Food};

#[derive(Debug, Copy, Clone)]
pub struct Ingredient {
    pub ingredient: Food::Ingredients,
}

impl Component for Ingredient {
    type Storage = DenseVecStorage<Self>;
}
