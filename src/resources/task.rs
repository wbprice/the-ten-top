use amethyst::ecs::prelude::Entity;

use crate::{
    components::{Dish, Ingredient, Destination},
};

#[derive(Debug, Copy, Clone)]
pub enum Tasks {
    TakeOrder { patron: Entity },
    DeliverOrder { patron: Entity, dish: Dish },
    FetchIngredient { ingredient: Ingredient },
    PrepIngredient { ingredient: Ingredient },
    PlateOrder { dish: Dish },
}

#[derive(Debug, Copy, Clone)]
pub enum Subtasks {
    MoveToEntity { entity: Entity },
    SetEntityOwner { entity: Entity, owner: Entity },
    MoveTo { destination: Destination }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    New,
    InProgress,
    Completed,
    Blocked
}
