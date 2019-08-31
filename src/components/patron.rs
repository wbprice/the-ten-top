use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Food;

pub struct Patron {
    pub satisfaction: u8,
    pub velocity: [f32; 2],
    pub craving: Food,
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}