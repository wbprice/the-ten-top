use crate::{
    components::Task,
    resources::{Dishes, Tasks},
};
use amethyst::ecs::Entity;

#[derive(Debug, Default)]
pub struct GameState {
    pub tasks: Vec<Task>,
    pub money: i32,
}

impl GameState {
    pub fn schedule_take_order(&mut self, entity: Entity) {
        self.tasks
            .push(Task::new(Tasks::TakeOrder { patron: entity }));
    }

    pub fn schedule_deliver_order(&mut self, entity: Entity, dish: Dishes) {
        self.tasks.push(Task::new(Tasks::DeliverOrder {
            patron: entity,
            dish,
        }));
    }
}
