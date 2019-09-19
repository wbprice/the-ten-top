use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::{Status, Subtask, Task};

#[derive(Debug)]
pub struct Assignment {
    pub task: Task,
    pub subtasks: Vec<Subtask>,
    pub subtask_index: usize,
    pub status: Status,
}

impl Assignment {
    pub fn new(task: Task) -> Assignment {
        Assignment {
            task,
            subtasks: vec![],
            subtask_index: 0,
            status: Status::New,
        }
    }
}

impl Component for Assignment {
    type Storage = DenseVecStorage<Self>;
}
