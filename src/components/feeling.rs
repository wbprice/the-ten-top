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

pub struct Feeling {
    pub symbol: Dish,
}

impl Component for Feeling {
    type Storage = DenseVecStorage<Self>;
}
