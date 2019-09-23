use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Cooked;

impl Component for Cooked {
    type Storage = DenseVecStorage<Self>;
}