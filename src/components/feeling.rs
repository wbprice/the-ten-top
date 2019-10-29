use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::Dishes;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Emotion {
    Happy,
    Sad,
    Neutral,
    Craving(Dishes),
}

#[derive(Debug, Copy, Clone)]
pub struct Feeling {
    pub symbol: Emotion,
}

impl Component for Feeling {
    type Storage = DenseVecStorage<Self>;
}
