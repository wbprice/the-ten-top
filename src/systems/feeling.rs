use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Emotion, Feeling, Foods, Patron, ThoughtBubble},
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
        for (patron_entity, _) in (&entities, &patrons).join() {
            // For each patron, spawn a thought bubble that shows what
            // they are thinking.

            // Does the thought bubble already exist?
            // If so, don't recreate it.
            let patron_feeling = feelings.get(patron_entity).unwrap();
            let thought_bubble_entity = match (&entities, &thought_bubbles, &parents)
                .join()
                .find(|(_, _, parent)| parent.entity.id() == patron_entity.id())
            {
                Some((thought_bubble_entity, _, _)) => Some(thought_bubble_entity),
                None => None,
            };

            match thought_bubble_entity {
                None => {
                    let mut thought_local = Transform::default();
                    thought_local.prepend_translation_y(20.0);

                    let thought_bubble = entities
                        .build_entity()
                        .with(ThoughtBubble {}, &mut thought_bubbles)
                        .with(
                            SpriteRender {
                                sprite_sheet: sprite_resource.sprite_sheet.clone(),
                                sprite_number: 12,
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

                    entities
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
                                sprite_number: 0,
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
                }
                Some(thought_bubble_entity) => {
                    if let Some(feeling_entity) = match (&entities, &feelings, &parents, &sprites)
                        .join()
                        .find(|(_, _, parent, _)| parent.entity.id() == thought_bubble_entity.id())
                    {
                        Some((feeling_entity, _, _, _)) => Some(feeling_entity),
                        None => None,
                    } {
                        let feeling = feelings.get(feeling_entity).unwrap();
                        if feeling.symbol != patron_feeling.symbol {
                            feelings
                                .insert(
                                    feeling_entity,
                                    Feeling {
                                        symbol: patron_feeling.symbol,
                                    },
                                )
                                .unwrap();

                            for (feeling, sprite, _) in (&feelings, &mut sprites, !&patrons).join()
                            {
                                match &feeling.symbol {
                                    Emotion::Craving(craving) => match craving {
                                        Foods::HotDog => {
                                            sprite.sprite_number = 20;
                                        }
                                        Foods::Hamburger => {
                                            sprite.sprite_number = 17;
                                        }
                                        _ => {
                                            sprite.sprite_number = 2;
                                        }
                                    },
                                    Emotion::Happy => {
                                        sprite.sprite_number = 0;
                                    }
                                    Emotion::Sad => {
                                        sprite.sprite_number = 1;
                                    }
                                    _ => {
                                        sprite.sprite_number = 2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
