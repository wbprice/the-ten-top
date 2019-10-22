use crate::resources::{Status, Subtasks, Tasks};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Task {
    pub activity: Tasks,
    pub subtasks: Box<Vec<Subtask>>,
    pub status: Status,
}

impl Task {
    pub fn new(activity: Tasks) -> Task {
        Task {
            activity,
            subtasks: Box::new(vec![]),
            status: Status::New,
        }
    }

    pub fn set_status(mut self, status: Status) {
        self.status = status;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Subtask {
    pub activity: Subtasks,
    pub status: Status,
}

impl Subtask {
    pub fn new(activity: Subtasks) -> Subtask {
        Subtask {
            activity,
            status: Status::New,
        }
    }
}

impl Component for Task {
    type Storage = DenseVecStorage<Self>;
}
