use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dish {
    Hamburger,
    HotDog,
    Taco,
    Elote,
    Takoyaki,
    Fishballs,
    BanhMi,
    Pho,
    Dumplings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ingredient {
    HamburgerBun,
    HamburgerPatty,
    Lettuce,
    Tomato,
    HotDogBun,
    HotDog,
}

#[derive(Debug)]
pub struct Food {
    pub dish: Dish,
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}
