use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Dish, Ingredient, Plate},
    resources::{Cookbook, Dishes, Ingredients},
};

pub struct PlateSystem;

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

            let ingredient_types: Vec<Ingredients> = ingredients_and_entity
                .iter()
                .map(|(_, _, ingredient)| ingredient.ingredient)
                .collect();

            // Do they make a hot dog?
            let hot_dog_ingredients: Vec<Ingredients> =
                vec![Ingredients::HotDogWeinerCooked, Ingredients::HotDogBun];

            if hot_dog_ingredients
                .into_iter()
                .all(|ingred| ingredient_types.contains(&ingred))
            {
                for (entity, parent, _) in ingredients_and_entity {
                    entities_to_remove.push(entity);
                    entities_to_create.push((plate_entity, Dishes::HotDog));
                }
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
