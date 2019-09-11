use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Destination {
    pub x: f32,
    pub y: f32
}

impl Component for Destination {
    type Storage = DenseVecStorage<Self>;
}