use amethyst::ecs::prelude::Entity;

use crate::components::{Dish, Ingredient};

#[derive(Debug, Copy, Clone)]
pub enum Task {
    TakeOrder { patron: Entity },
    DeliverOrder { patron: Entity, dish: Dish },
    FetchIngredient { ingredient: Ingredient },
    PrepIngredient { ingredient: Ingredient },
    PlateOrder { dish: Dish },
}
