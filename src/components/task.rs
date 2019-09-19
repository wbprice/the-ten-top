use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::resources::{Status, Subtasks, Tasks};

#[derive(Debug, Copy, Clone)]
pub struct Subtask {
    pub activity: Subtasks,
    pub status: Status,
}

#[derive(Debug)]
pub struct Task {
    pub activity: Tasks,
    pub subtasks: Vec<Subtask>,
    pub status: Status,
}

impl Task {
    pub fn new(activity: Tasks) -> Task {
        Task {
            activity,
            subtasks: vec![],
            status: Status::New,
        }
    }
}

impl Component for Task {
    type Storage = DenseVecStorage<Self>;
}
