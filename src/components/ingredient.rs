use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Copy, Clone)]
pub struct Ingredient {
    pub ingredient: Ingredients,
}

impl Component for Ingredient {
    type Storage = DenseVecStorage<Self>;
}
