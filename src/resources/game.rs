use crate::{components::Dish, resources::PatronOrder};
use amethyst::ecs::Entity;

#[derive(Debug, Default)]
pub struct GameState {
    pub patron_orders: Vec<PatronOrder>,
    pub money: i32,
}

impl GameState {
    pub fn make_order(&mut self, patron: Entity, dish: Dish) {
        self.patron_orders.push(PatronOrder { patron, dish });
    }
}
