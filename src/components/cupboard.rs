use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::{
    components::{Ingredient}
};

#[derive(Debug)]
pub struct Cupboard {
    pub ingredient: Ingredient,
    pub cooldown: u8
}

impl Cupboard {
    pub fn new(ingredient: Ingredient) -> Cupboard {
        Cupboard {
            ingredient,
            cooldown: 0
        }
    }
}

impl Component for Cupboard {
    type Storage = DenseVecStorage<Self>;
}
