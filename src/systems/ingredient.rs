use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Ingredients, Ingredient};

pub struct IngredientSystem;

impl<'s> System<'s> for IngredientSystem {
    type SystemData = (ReadStorage<'s, Ingredient>, WriteStorage<'s, SpriteRender>);

    fn run(&mut self, (ingredients, mut sprites): Self::SystemData) {
        for (ingredient, sprite) in (&ingredients, &mut sprites).join() {
            // What kind of food is this?
            match ingredient.ingredient {
                Ingredients::HotDogBun => {
                    sprite.sprite_number = 14;
                }
                _ => {
                    sprite.sprite_number = 13;
                }
            }
        }
    }
}