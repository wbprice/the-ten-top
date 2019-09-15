use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Dish;

#[derive(Debug)]
pub enum Emotion {
    Happy,
    Sad,
    Neutral,
    Craving(Dish),
}

#[derive(Debug)]
pub struct Feeling {
    pub symbol: Emotion,
}

impl Component for Feeling {
    type Storage = DenseVecStorage<Self>;
}
