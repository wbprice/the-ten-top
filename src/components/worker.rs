use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Worker;

impl Component for Worker {
    type Storage = DenseVecStorage<Self>;
}
