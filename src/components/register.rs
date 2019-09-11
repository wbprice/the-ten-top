use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Register;

impl Component for Register {
    type Storage = DenseVecStorage<Self>;
}