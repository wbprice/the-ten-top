use amethyst::{
    ecs::prelude::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{
        Food, 
        Dishes
    },
    resources::{
        SpriteResource,
        Food
    }
};

pub struct DishSystem;

impl<'s> System<'s> for DishSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Dish>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, SpriteResource>,
    );

    fn run(&mut self, (entities, dishes, mut sprites, sprite_resource): Self::SystemData) {
        // One time food setup
        // Identity new foods that need sprites

        let mut sprites_to_insert: Vec<(Entity, SpriteRender)> = vec![];

        for (entity, dish, _) in (&entities, &dishes, !&sprites).join() {
            let sprite_sheet = sprite_resource.sprite_sheet.clone();

            match dishes.dish {
                Food::Dishes(Dishes::HotDog) => {
                    sprites_to_insert.push((
                        entity,
                        SpriteRender {
                            sprite_sheet,
                            sprite_number: 20,
                        },
                    ));
                }
                _ => {
                    sprites_to_insert.push((
                        entity,
                        SpriteRender {
                            sprite_sheet,
                            sprite_number: 17,
                        },
                    ));
                }
            }
        }

        for (entity, sprite) in sprites_to_insert {
            sprites.insert(entity, sprite).unwrap();
        }
    }
}
