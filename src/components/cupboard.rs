use crate::resources::Ingredients;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Cupboard {
    pub ingredient: Ingredients,
    pub cooldown: f32,
}

impl Cupboard {
    pub fn new(ingredient: Ingredients) -> Cupboard {
        Cupboard {
            ingredient,
            cooldown: 0.0,
        }
    }
}

impl Component for Cupboard {
    type Storage = DenseVecStorage<Self>;
}
