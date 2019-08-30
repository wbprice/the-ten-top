use amethyst::ecs::prelude::{
    Component,
    DenseVecStorage
}

enum Task {
    TAKING_ORDER,
    CLEANING,
    COOKING
}

struct Worker {
    satisfaction: u8,
    velocity: f[f32; 2],
    task: Task
}

impl Component for Worker {
    type Storage = DenseVecStorage<Self>;
}