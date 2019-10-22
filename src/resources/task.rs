use amethyst::ecs::prelude::Entity;

use crate::components::{Destination, Emotion, Foods, Ingredients};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tasks {
    TakeOrder {
        patron: Entity,
    },
    DeliverOrder {
        patron: Entity,
        food: Foods,
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
        food: Foods,
    },
    GiveOrder {
        register: Entity,
        food: Foods,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Subtasks {
    MoveToEntity { entity: Entity },
    SetEntityOwner { entity: Entity, owner: Entity },
    MoveTo { destination: Destination },
    WaitForWorker,
    SubmitOrder { food: Foods },
    WaitForOrder { food: Foods },
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
