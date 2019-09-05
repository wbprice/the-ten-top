use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
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

pub struct Food {
    pub dish: Dish,
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}
