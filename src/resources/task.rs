use amethyst::ecs::prelude::Entity;

use crate::components::{Destination, Foods, Food, Emotion, Ingredient};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tasks {
    TakeOrder { patron: Entity },
    DeliverOrder { patron: Entity, food: Foods },
    FetchIngredient { ingredient: Ingredient },
    PrepIngredient { ingredient: Ingredient },
    PlateOrder { food: Food },
    MakeOrder { register: Entity, food: Food },
}

#[derive(Debug, Copy, Clone)]
pub enum Subtasks {
    MoveToEntity { entity: Entity },
    SetEntityOwner { entity: Entity, owner: Entity },
    MoveTo { destination: Destination },
    WaitForWorker,
    SubmitOrder { food: Food },
    WaitForOrder { food: Food },
    UpdateFeeling { symbol: Emotion },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    New,
    InProgress,
    Completed,
    Blocked,
}
