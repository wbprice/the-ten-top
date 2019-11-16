use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Dish, Ingredient, Plate},
    resources::{Cookbook, Dishes, Food, Ingredients},
};

pub struct PlateSystem;

fn get_dish_from_ingredients(
    cookbook: &Cookbook,
    ingredients: &Vec<Ingredients>,
) -> Option<Dishes> {
    // Do all the ingredients contribute to the same result?
    if !ingredients.is_empty() {
        let first_ingredient = ingredients[0];
        let potential_dishes = cookbook.makes(Food::Ingredients(first_ingredient));

        for ingredient in ingredients.into_iter() {
            let ingredient_makes = cookbook.makes(Food::Ingredients(*ingredient));
            for other_ingred in ingredient_makes.iter() {
                if !potential_dishes.contains(other_ingred) {
                    return None;
                }
            }
        }

        return Some(potential_dishes[0]);
    }

    None
}

impl<'s> System<'s> for PlateSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Parent>,
        ReadStorage<'s, Plate>,
        ReadStorage<'s, Ingredient>,
        WriteStorage<'s, Dish>,
        WriteStorage<'s, Transform>,
        Read<'s, Cookbook>,
    );

    fn run(
        &mut self,
        (entities, mut parents, plates, ingredients, mut dishes, mut locals, cookbook): Self::SystemData,
    ) {
        let mut entities_to_remove: Vec<Entity> = vec![];
        let mut entities_to_create: Vec<(Entity, Dishes)> = vec![];

        for (plate_entity, plate) in (&entities, &plates).join() {
            // Iterate through the plates.
            // What ingredients have this plate as a parent?
            let ingredients_and_entity: Vec<(Entity, &mut Parent, &Ingredient)> =
                (&entities, &mut parents, &ingredients)
                    .join()
                    .filter(|(_, parent, _)| &parent.entity == &plate_entity)
                    .collect();

            let mut ingredient_types: Vec<Ingredients> = ingredients_and_entity
                .iter()
                .map(|(_, _, ingredient)| ingredient.ingredient)
                .collect();

            // If the ingredients all contribute to the same dish
            if let Some(dish) = get_dish_from_ingredients(&cookbook, &ingredient_types) {
                // and all ingredients are in place:
                let required_ingredients = cookbook.ingredients(Food::Dishes(dish));
                let all_ingredients_found = required_ingredients
                    .iter()
                    .all(|ingredient| ingredient_types.contains(ingredient));

                if all_ingredients_found {
                    for (entity, parent, _) in ingredients_and_entity {
                        entities_to_remove.push(entity);
                        entities_to_create.push((plate_entity, Dishes::HotDog));
                    }

                    for entity in &entities_to_remove {
                        entities.delete(*entity).unwrap();
                    }

                    for entity in &entities_to_create {
                        entities
                            .build_entity()
                            .with(
                                Dish {
                                    dish: Dishes::HotDog,
                                },
                                &mut dishes,
                            )
                            .with(
                                Parent {
                                    entity: plate_entity,
                                },
                                &mut parents,
                            )
                            .with(Transform::default(), &mut locals)
                            .build();
                    }
                }
            }
        }
    }
}
