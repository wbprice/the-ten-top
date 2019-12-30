use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::{components::Ingredient, resources::Dishes};

#[derive(Debug)]
pub struct Plate {
    stacked_ingredients: Vec<Ingredient>,
    target_food: Option<Dishes>,
}

impl Plate {
    pub fn new() -> Plate {
        Plate {
            stacked_ingredients: vec![],
            target_food: None,
        }
    }
}

impl Component for Plate {
    type Storage = DenseVecStorage<Self>;
}
