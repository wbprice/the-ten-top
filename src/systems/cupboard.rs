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

        let cupboard_entities : Vec<Entity> = (&entities, &cupboards)
            .join()
            .map(|(entity, _)| entity)
            .collect();

        let ingredient_parents : Vec<Entity> = (&entities, &mut ingredients, &mut parents)
            .join()
            .map(|(_, _, parent)| parent.entity)
            .collect();

        let empty_cupboards = cupboard_entities
            .iter()
            .filter(|cupboard_entity| !ingredient_parents.contains(cupboard_entity));

        // Cupboards that don't have

        for entity in empty_cupboards {
            let cupboard = cupboards.get_mut(*entity).unwrap();

            if cupboard.cooldown <= 0.0 {
                // spawn ingredient of type cupboard.ingredient
                // reset the cooldown to 10.0 seconds
                let mut ingredient_local = Transform::default();
                ingredient_local.prepend_translation_y(12.0);
                ingredient_local.prepend_translation_z(0.1);

                entities
                    .build_entity()
                    .with(Ingredient {
                        ingredient: cupboard.ingredient
                    }, &mut ingredients)
                    .with(SpriteRender {
                        sprite_sheet: sprite_resource.sprite_sheet.clone(),
                        sprite_number: 12
                    }, &mut sprites)
                    .with(Parent {
                        entity: *entity
                    }, &mut parents)
                    .with(ingredient_local, &mut locals)
                    .build();

                cupboard.cooldown = 10.0;
                dbg!("An ingredient was spawned");
            } else {
                // tick down cooldown
                cupboard.cooldown = cupboard.cooldown - time.delta_seconds();
                dbg!("Decrement cooldown");
            }
        }
    }
}