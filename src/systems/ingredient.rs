use amethyst::{
    ecs::prelude::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Ingredient, Ingredients},
    resources::SpriteResource,
};

pub struct IngredientSystem;

impl<'s> System<'s> for IngredientSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        ReadStorage<'s, Ingredient>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (entities, sprite_resource, ingredients, mut sprites): Self::SystemData) {
        // One time ingredient sprite setup
        // Identify new ingredients that need sprites
        let new_ingredients: Vec<(Entity, Ingredients)> = (&entities, &ingredients, !&sprites)
            .join()
            .map(|(entity, ingredient, _)| (entity, ingredient.ingredient))
            .collect();

        // Iterate over new ingredients, adding new sprites
        for (entity, ingredient) in new_ingredients {
            let sprite_sheet = sprite_resource.sprite_sheet.clone();

            // What kind of ingredient is this?
            match ingredient {
                Ingredients::HotDogWeiner => {
                    sprites
                        .insert(
                            entity,
                            SpriteRender {
                                sprite_sheet,
                                sprite_number: 18,
                            },
                        )
                        .unwrap();
                }
                Ingredients::HotDogBun => {
                    sprites
                        .insert(
                            entity,
                            SpriteRender {
                                sprite_sheet,
                                sprite_number: 19,
                            },
                        )
                        .unwrap();
                }
                _ => {
                    sprites
                        .insert(
                            entity,
                            SpriteRender {
                                sprite_sheet,
                                sprite_number: 3,
                            },
                        )
                        .unwrap();
                }
            }
        }
    }
}
