use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, ReadExpect, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    resources::SpriteResource,
    components::{Patron, Dish, Emotion, Feeling, ThoughtBubble}
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

    fn run(&mut self, (entities, sprite_resource, patrons, mut sprites, mut feelings, mut thought_bubbles, mut parents, mut locals): Self::SystemData) {
        for (patron_entity, patron, patron_feeling) in (&entities, &patrons, &mut feelings).join() {
            // For each patron, spawn a thought bubble that shows what
            // they are thinking.

            // Does the thought bubble already exist?
            // If so, don't recreate it.
            // TODO

            let mut should_create_thought_bubble = true;
            for (entity, thought_bubble, parent) in (&entities, &thought_bubbles, &parents).join() {
                if parent.entity.id() == patron_entity.id() {
                    should_create_thought_bubble = false;
                }
            }

            if !should_create_thought_bubble {
                break;
            }

            let mut thought_local = Transform::default();
            thought_local.prepend_translation_y(20.0);

            let thought_bubble = entities.build_entity()
                .with(ThoughtBubble {}, &mut thought_bubbles)
                .with(SpriteRender {
                    sprite_sheet: sprite_resource.sprite_sheet.clone(),
                    sprite_number: 6
                }, &mut sprites)
                .with(Parent {
                    entity: patron_entity
                }, &mut parents)
                .with(thought_local, &mut locals)
                .build();

            let mut feeling_local = Transform::default();
            feeling_local.prepend_translation_y(3.0);
            feeling_local.prepend_translation_z(0.01);
            
            // let feeling = entities.build_entity()
            //     .with(patron_feeling, Self::SystemData::[2])
            //     .with(Parent {
            //         entity: thought_bubble
            //     }, &mut parents)
            //     .with(feeling_local, &mut locals)
            //     .build();
        }
    }
}
