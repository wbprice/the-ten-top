use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Dish, Emotion, Feeling, Patron, ThoughtBubble},
    resources::SpriteResource,
};

pub struct MoveFeelingSystem;

impl<'s> System<'s> for MoveFeelingSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        ReadStorage<'s, Patron>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Feeling>,
        WriteStorage<'s, ThoughtBubble>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_resource,
            patrons,
            mut sprites,
            mut feelings,
            mut thought_bubbles,
            mut parents,
            mut locals,
        ): Self::SystemData,
    ) {
        for (patron_entity, patron) in (&entities, &patrons).join() {
            // For each patron, spawn a thought bubble that shows what
            // they are thinking.

            // Does the thought bubble already exist?
            // If so, don't recreate it.
            let mut should_create_thought_bubble = true;
            for (entity, thought_bubble, parent) in (&entities, &thought_bubbles, &parents).join() {
                if parent.entity.id() == patron_entity.id() {
                    should_create_thought_bubble = false;
                }
            }

            if should_create_thought_bubble {
                let mut thought_local = Transform::default();
                thought_local.prepend_translation_y(20.0);

                let thought_bubble = entities
                    .build_entity()
                    .with(ThoughtBubble {}, &mut thought_bubbles)
                    .with(
                        SpriteRender {
                            sprite_sheet: sprite_resource.sprite_sheet.clone(),
                            sprite_number: 6,
                        },
                        &mut sprites,
                    )
                    .with(
                        Parent {
                            entity: patron_entity,
                        },
                        &mut parents,
                    )
                    .with(thought_local, &mut locals)
                    .build();

                let patron_feeling = feelings.get(patron_entity).unwrap();
                let mut feeling_local = Transform::default();
                feeling_local.prepend_translation_y(3.0);
                feeling_local.prepend_translation_z(0.01);

                // Does the feeling entity already exist?
                // If so, don't recreate it.
                let mut should_create_emotion = true;
                for (feeling_entity, feeling, parent) in (&entities, &feelings, &parents).join() {
                    if parent.entity.id() == feeling_entity.id() {
                        should_create_emotion = false;
                    }
                }

                if should_create_emotion {
                    let feeling = entities
                        .build_entity()
                        .with(
                            Feeling {
                                symbol: patron_feeling.symbol,
                            },
                            &mut feelings,
                        )
                        .with(
                            SpriteRender {
                                sprite_sheet: sprite_resource.sprite_sheet.clone(),
                                sprite_number: 8,
                            },
                            &mut sprites,
                        )
                        .with(
                            Parent {
                                entity: thought_bubble,
                            },
                            &mut parents,
                        )
                        .with(feeling_local, &mut locals)
                        .build();

                    break;
                }
            }


            // Update feeling symbols as needed.
            let patron_feeling = feelings.get(patron_entity).unwrap();
            for (thought_bubble_entity, thought_bubble, parent) in (&entities, &thought_bubbles, &parents).join() {
                // If the parent component points to the patron entity...
                if parent.entity.id() == patron_entity.id() {
                    // Get the feeling for this thought bubble
                    for (feeling_entity, feeling, parent, sprite) in (&entities, &mut feelings, &parents, &mut sprites).join() {
                        // If the parent component points to the thought bubble entity...
                        if parent.entity.id() == thought_bubble_entity.id() {
                            if feeling.symbol != patron_feeling.symbol {
                                // feelings_to_update.push((feeling_entity, patron_feeling.symbol));
                                feelings.insert(feeling_entity, Feeling {
                                    symbol: patron_feeling.symbol
                                }).unwrap();

                                // match &feeling.symbol {
                                //     Emotion::Craving(craving) => {
                                //         match craving {
                                //             Dish::HotDog => {
                                //                 sprite.sprite_number = 7;
                                //             }
                                //             Dish::Hamburger => {
                                //                 sprite.sprite_number = 8;
                                //             }
                                //             _ => {
                                //                 sprite.sprite_number = 13;
                                //             }
                                //         }
                                //     }
                                //     Emotion::Happy => {
                                //         sprite.sprite_number = 11;
                                //     },
                                //     Emotion::Sad => {
                                //         sprite.sprite_number = 12;
                                //     }
                                //     _ => {
                                //         sprite.sprite_number = 13;
                                //     }
                                // }
                           } 
                        }
                    }    
                }
            }

            for (feeling_entity, feeling, sprite) in (&entities, &feelings, &sprites).join() {
            }
        }
    }
}
