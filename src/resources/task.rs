use amethyst::ecs::prelude::Entity;

use crate::{
    common::Point,
    components::{Dish, Ingredient},
};

#[derive(Debug, Copy, Clone)]
pub enum Task {
    TakeOrder { patron: Entity },
    DeliverOrder { patron: Entity, dish: Dish },
    FetchIngredient { ingredient: Ingredient },
    PrepIngredient { ingredient: Ingredient },
    MoveToEntity { entity: Entity },
    MoveTo { point: Point },
    PlateOrder { dish: Dish },
}

#[derive(Debug, Copy, Clone)]
pub enum Subtask {
    MoveToEntity { patron: Entity },
    MoveTo { point: Point },
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    New,
    InProgress,
    Completed,
}
