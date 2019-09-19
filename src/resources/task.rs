use amethyst::ecs::prelude::Entity;

use crate::{
    common::Point,
    components::{Dish, Ingredient},
};

#[derive(Debug, Copy, Clone)]
pub enum Tasks {
    TakeOrder { patron: Entity },
    DeliverOrder { patron: Entity, dish: Dish },
    FetchIngredient { ingredient: Ingredient },
    PrepIngredient { ingredient: Ingredient },
    MoveToEntity { entity: Entity },
    MoveTo { point: Point },
    PlateOrder { dish: Dish },
}

#[derive(Debug, Copy, Clone)]
pub enum Subtasks {
    MoveToEntity { entity: Entity },
    PickUpEntity { entity: Entity },
    SetEntityOwner { entity: Entity, owner: Entity }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    New,
    InProgress,
    Completed,
    Blocked
}
