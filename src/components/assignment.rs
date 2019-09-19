use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::{
    Task,
    Subtask,
    Status
};

#[derive(Debug, Copy, Clone)]
pub struct Assignment {
    pub task: Task,
    pub status: Status
}

impl Assignment {
    pub fn new(task: Task) -> Assignment {
        Assignment {
            task,
            status: Status::New
        }
    }
}

impl Component for Assignment {
    type Storage = DenseVecStorage<Self>;
}
