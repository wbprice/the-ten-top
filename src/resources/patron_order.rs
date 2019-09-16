use amethyst::ecs::{
    Entity
};

use crate::components::Dish;

#[derive(Debug)]
pub struct PatronOrder {
    pub patron: Entity,
    pub dish: Dish
}
