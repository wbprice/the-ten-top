use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Velocity;

#[derive(Debug, Copy, Clone)]
pub struct Destination {
    pub x: f32,
    pub y: f32,
}

impl Component for Destination {
    type Storage = DenseVecStorage<Self>;
}
