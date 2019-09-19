use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::{
    Task,
    Subtask
};

pub struct Assignment {
    pub task: Task,
    pub subtasks: Vec<Subtask>,
    pub complete: bool
}

impl Component for Assignment {
    type Storage = DenseVecStorage<Self>;
}
