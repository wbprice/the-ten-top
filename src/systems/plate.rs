use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Food, Foods, Ingredient, Ingredients, Plate};

pub struct PlateSystem;

impl<'s> System<'s> for PlateSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Parent>,
        ReadStorage<'s, Plate>,
        WriteStorage<'s, Ingredient>,
        WriteStorage<'s, Food>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, mut parents, plates, mut ingredients, mut foods, mut locals): Self::SystemData,
    ) {
        let mut ingredients_to_remove: Vec<Entity> = vec![];
        let mut foods_to_create: Vec<(Entity, Foods)> = vec![];

        for (plate_entity, plate) in (&entities, &plates).join() {
            // Iterate through the plates.
            // What ingredients have this plate as a parent?
            let ingredients_and_entity: Vec<(Entity, &Parent, &Ingredient)> =
                (&entities, &parents, &ingredients)
                    .join()
                    .filter(|(_, parent, _)| parent.entity == plate_entity)
                    .collect();

            let ingredient_types: Vec<Ingredients> = ingredients_and_entity
                .iter()
                .map(|(_, _, ingredient)| ingredient.ingredient)
                .collect();

            // Do they make a hot dog?
            let hot_dog_ingredients: Vec<Ingredients> =
                vec![Ingredients::HotDogWeiner, Ingredients::HotDogBun];

            if hot_dog_ingredients
                .into_iter()
                .all(|ingred| ingredient_types.contains(&ingred))
            {
                dbg!("Remove the ingredients");
                dbg!("Make a food component here!");
                for (entity, _, _) in ingredients_and_entity {
                    ingredients_to_remove.push(entity);
                }
                foods_to_create.push((plate_entity, Foods::HotDog));
            }
        }

        for entity in ingredients_to_remove {
            ingredients.remove(entity).unwrap();
        }
        for (parent, food) in foods_to_create {
            entities
                .build_entity()
                .with(Transform::default(), &mut locals)
                .with(Parent { entity: parent }, &mut parents)
                .with(Food { food: food }, &mut foods)
                .build();
        }
    }
}
