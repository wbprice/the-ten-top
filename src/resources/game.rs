use crate::{components::Dish, resources::Task};
use amethyst::ecs::Entity;

#[derive(Debug, Default)]
pub struct GameState {
    pub tasks: Vec<Task>,
    pub money: i32,
}

impl GameState {
    pub fn schedule_take_order(&mut self, patron: Entity) {
        self.tasks.push(Task::TakeOrder {
            patron
        });
    }
}
