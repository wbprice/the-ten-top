use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Dish, Emotion, Feeling, ThoughtBubble};

pub struct MoveFeelingSystem;

impl<'s> System<'s> for MoveFeelingSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Feeling>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (entities, feelings, mut sprites): Self::SystemData) {
        for (entity, feeling, sprite) in (&entities, &feelings, &mut sprites).join() {
            match &feeling.symbol {
                Emotion::Craving(craving) => {
                    match craving {
                        Dish::HotDog => {
                            sprite.sprite_number = 7;
                        }
                        Dish::Hamburger => {
                            sprite.sprite_number = 8;
                        }
                        _ => {
                            // everything else
                        }
                    }
                }
                Emotion::Happy => {
                    sprite.sprite_number = 1;
                }
                _ => {
                    // everything else
                }
            }
        }
    }
}
