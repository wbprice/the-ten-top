use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum Task {
    TakingOrder,
    Cleaning,
    Cooking,
}

pub struct Worker {
    satisfaction: u8,
    velocity: [f32; 2],
    task: Task,
}

impl Component for Worker {
    type Storage = DenseVecStorage<Self>;
}
