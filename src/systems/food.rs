use amethyst::{
    ecs::prelude::{Join, ReadStorage, Entities, Entity, ReadExpect, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Food, Foods},
    resources::{SpriteResource}
};

pub struct FoodSystem;

impl<'s> System<'s> for FoodSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, SpriteResource>
    );

    fn run(&mut self, (entities, foods, mut sprites, sprite_resource): Self::SystemData) {
        // One time food setup
        // Identity new foods that need sprites

        let mut sprites_to_insert : Vec<(Entity, SpriteRender)> = vec![];

        for (entity, food, _) in (&entities, &foods, !&sprites).join() {
            let sprite_sheet = sprite_resource.sprite_sheet.clone();

            match food.food {
                Foods::HotDog => {
                    sprites_to_insert.push((entity, SpriteRender {
                        sprite_sheet,
                        sprite_number: 20,
                    }));
                }
                _ => {
                    sprites_to_insert.push((entity, SpriteRender {
                        sprite_sheet,
                        sprite_number: 17,
                    }));
                }
            }
        }

        for (entity, sprite) in sprites_to_insert {
            sprites.insert(entity, sprite).unwrap();
        }
    }
}
