use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::Task;

pub struct Assignment {
    task: Task
}

impl Component for Assignment {
    type Storage = DenseVecStorage<Self>;
}
