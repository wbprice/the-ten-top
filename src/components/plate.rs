use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Plate;

impl Component for Plate {
    type Storage = DenseVecStorage<Self>;
}
