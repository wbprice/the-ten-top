use amethyst::ecs::prelude::Entity;

use crate::{
    components::{Destination, Emotion},
    resources::{Dishes, Ingredients},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tasks {
    TakeOrder {
        patron: Entity,
    },
    DeliverOrder {
        patron: Entity,
        dish: Dishes,
    },
    FetchIngredient {
        ingredient: Ingredients,
    },
    PrepIngredient {
        ingredient: Ingredients,
    },
    PlateIngredient {
        ingredient: Ingredients,
        plate: Entity,
    },
    PrepOrder {
        dish: Dishes,
    },
    GiveOrder {
        register: Entity,
        dish: Dishes,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Subtasks {
    MoveToEntity { entity: Entity },
    SetEntityOwner { entity: Entity, owner: Entity },
    MoveTo { destination: Destination },
    WaitForWorker,
    SubmitOrder { dish: Dishes },
    WaitForOrder { dish: Dishes },
    UpdateFeeling { symbol: Emotion },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    New,
    Actionable,
    InProgress,
    Completed,
    Blocked,
}
