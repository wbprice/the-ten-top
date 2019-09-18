use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::Task;

pub struct Assignment {
    pub task: Task,
}

impl Component for Assignment {
    type Storage = DenseVecStorage<Self>;
}
