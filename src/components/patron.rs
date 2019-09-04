use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Food;

#[derive(Debug)]
pub struct Patron {
    pub satisfaction: u8,
    pub velocity: [f32; 2]
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}
