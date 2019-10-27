use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadExpect, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Cooked, Ingredient, Ingredients, Stove},
    resources::SpriteResource,
};

pub struct StoveSystem;

impl<'s> System<'s> for StoveSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Stove>,
        WriteStorage<'s, Ingredient>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cooked>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_resource,
            mut sprites,
            mut stoves,
            mut ingredients,
            mut parents,
            mut locals,
            mut cooked,
            time,
        ): Self::SystemData,
    ) {
        // Find out which stove entities have food in them.
        // If a stove has food on it, it should cook it

        let stove_entities: Vec<Entity> = (&entities, &stoves)
            .join()
            .map(|(entity, _)| entity)
            .collect();

        let ingredient_parents: Vec<Entity> = (&entities, &mut ingredients, &mut parents)
            .join()
            .map(|(_, _, parent)| parent.entity)
            .collect();

        let stoves_in_use = stove_entities
            .into_iter()
            .filter(|stove_entity| ingredient_parents.contains(stove_entity));

        // Cupboards that are empty should regenerate their ingredient after 10 seconds.
        for stove_entity in stoves_in_use {
            let stove = stoves.get_mut(stove_entity).unwrap();

            if stove.cook_time <= 0.0 {
                // Update ingredient to be of the cooked variety.
                // reset the cooldown to 10.0 seconds

                // Find the ingredient entity of the food on the stove.
                let ingredient_entities: Vec<Entity> = (&entities, &mut ingredients, &mut parents)
                    .join()
                    .filter(|(_, _, parent)| parent.entity == stove_entity)
                    .map(|(ingredient_entity, _, _)| ingredient_entity)
                    .collect();

                let ingredient_entity: Entity = ingredient_entities[0];

                // Replace the ingredient with a cooked counterpart
                ingredients.insert(ingredient_entity, Ingredient {
                    ingredient: Ingredients::HotDogWeinerCooked
                }).unwrap();

                stove.cook_time = 6.0;
            } else {
                // tick down cooldown
                stove.cook_time = stove.cook_time - time.delta_seconds();
            }
        }
    }
}
