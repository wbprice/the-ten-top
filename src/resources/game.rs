use amethyst::ecs::Entity;
use crate::{
    resources::Tasks,
    components::{Dish}
};

#[derive(Debug, Default)]
pub struct GameState {
    pub tasks: Vec<Tasks>,
    pub money: i32,
}

impl GameState {
    pub fn schedule_take_order(&mut self, entity: Entity) {
        self.tasks.push(Tasks::TakeOrder { patron: entity });
    }

    pub fn schedule_deliver_order(&mut self, entity: Entity, dish: Dish) {
        self.tasks.push(Tasks::DeliverOrder { patron: entity, dish});
    }
}
