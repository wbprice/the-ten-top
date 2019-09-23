use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadExpect, Read, ReadStorage, System, Write, WriteStorage},
    renderer::SpriteRender
};

use crate::{
    components::{
        Cupboard,
        Ingredient,
        Ingredients
    },
    resources::{
        SpriteResource
    }
};

pub struct CupboardSystem;

impl<'s> System<'s> for CupboardSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Cupboard>,
        WriteStorage<'s, Ingredient>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (entities, sprite_resource, mut sprites, mut cupboards, mut ingredients, mut parents, mut locals, time): Self::SystemData) {

        let mut cupboards_to_spawn_ingredients : Vec<(Entity, Ingredients)> = vec![];

        for (entity, cupboard, _) in (&entities, &mut cupboards, !&parents).join() {
            if cupboard.cooldown <= 0.0 {
                // spawn ingredient of type cupboard.ingredient
                // reset the cooldown to 10.0 seconds
                cupboards_to_spawn_ingredients.push((entity, cupboard.ingredient));
                cupboard.cooldown = 10.0;
            } else {
                // tick down cooldown
                cupboard.cooldown = cupboard.cooldown - time.delta_seconds();
            }
        }

        for (entity, ingredient) in cupboards_to_spawn_ingredients {
            let mut ingredient_local = Transform::default();
            ingredient_local.prepend_translation_y(24.0);

            entities
                .build_entity()
                .with(Ingredient {
                    ingredient
                }, &mut ingredients)
                .with(SpriteRender {
                    sprite_sheet: sprite_resource.sprite_sheet.clone(),
                    sprite_number: 12
                }, &mut sprites)
                .with(Parent {
                    entity
                }, &mut parents)
                .with(ingredient_local, &mut locals)
                .build();
        }
    }
}