use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Dish, Food};

pub struct FoodSystem;

impl<'s> System<'s> for FoodSystem {
    type SystemData = (ReadStorage<'s, Food>, WriteStorage<'s, SpriteRender>);

    fn run(&mut self, (foods, mut sprites): Self::SystemData) {
        for (food, sprite) in (&foods, &mut sprites).join() {
            // What kind of food is this?
            match food.dish {
                Dish::HotDog => {
                    sprite.sprite_number = 14;
                }
                _ => {
                    sprite.sprite_number = 13;
                }
            }
        }
    }
}
