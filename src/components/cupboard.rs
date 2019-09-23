use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::{
    components::{Ingredients}
};

#[derive(Debug)]
pub struct Cupboard {
    pub ingredient: Ingredients,
    pub cooldown: u8
}

impl Cupboard {
    pub fn new(ingredient: Ingredients) -> Cupboard {
        Cupboard {
            ingredient,
            cooldown: 0
        }
    }
}

impl Component for Cupboard {
    type Storage = DenseVecStorage<Self>;
}
