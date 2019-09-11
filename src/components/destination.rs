use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Destination {
    pub destination: [f32; 2]
}

impl Component for Destination {
    type Storage = DenseVecStorage<Self>;
}