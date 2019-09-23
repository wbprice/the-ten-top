use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Foods;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Emotion {
    Happy,
    Sad,
    Neutral,
    Craving(Foods),
}

#[derive(Debug, Copy, Clone)]
pub struct Feeling {
    pub symbol: Emotion,
}

impl Component for Feeling {
    type Storage = DenseVecStorage<Self>;
}
