use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
