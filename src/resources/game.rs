use crate::resources::Task;
use amethyst::ecs::Entity;

#[derive(Debug, Default)]
pub struct GameState {
    pub tasks: Vec<Task>,
    pub money: i32,
}

impl GameState {
    pub fn schedule_take_order(&mut self, entity: Entity) {
        self.tasks.push(Task::TakeOrder { patron: entity });
    }

    pub fn schedule_move_to_entity(&mut self, entity: Entity) {
        self.tasks.push(Task::MoveToEntity { entity });
    }
}
