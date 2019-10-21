use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, Read, System, WriteStorage},
    renderer::SpriteRender
};

use crate::{
    components::{
        Plate,
        Ingredient,
        Ingredients
    }
};

pub struct PlateSystem;

impl<'s> System<'s> for PlateSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Plate>,
        WriteStorage<'s, Ingredient>
    );

    fn run(&mut self, (entities, parents, plates, ingredients): Self::SystemData) {
        for (plate_entity, plate) in (&entities, &plates).join() {
            // Iterate through the plates.
            // What ingredients have this plate as a parent?
            let ingredients_and_entity : Vec<(Entity, &Parent, &Ingredient)> = (&entities, &parents, &ingredients)
                .join()
                .filter(|(_, parent, _)| parent.entity == plate_entity)
                .collect();

            let ingredient_types : Vec<Ingredients> = ingredients_and_entity
                .iter()
                .map(|(_, _, ingredient)| ingredient.ingredient)
                .collect();

            // Do they make a hot dog?
            let hot_dog_ingredients: Vec<Ingredients> = vec![
                Ingredients::HotDogWeiner,
                Ingredients::HotDogBun,
            ];

            if hot_dog_ingredients.into_iter().all(|ingred| ingredient_types.contains(&ingred)) {
                dbg!("Make a food component here!");
            }
        }
    }
}