use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::Ingredients;

#[derive(Debug, Copy, Clone)]
pub struct Ingredient {
    pub ingredient: Ingredients,
}

impl Component for Ingredient {
    type Storage = DenseVecStorage<Self>;
}
