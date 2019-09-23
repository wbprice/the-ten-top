use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Foods {
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

#[derive(Debug)]
pub struct Food {
    pub food: Foods,
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}
