use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ingredients {
    HamburgerBun,
    HamburgerPatty,
    Lettuce,
    Tomato,
    HotDogBun,
    HotDog,
}

#[derive(Debug, Copy, Clone)]
pub struct Ingredient {
    pub ingredient: Ingredients,
}

impl Component for Ingredient {
    type Storage = DenseVecStorage<Self>;
}