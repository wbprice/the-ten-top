use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Patron, Dish, Emotion, Feeling, ThoughtBubble};

pub struct MoveFeelingSystem;

impl<'s> System<'s> for MoveFeelingSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Patron>,
        ReadStorage<'s, Feeling>
    );

    fn run(&mut self, (entities, patrons, feelings): Self::SystemData) {
        for (patron_entity, patron, patron_feeling) in (&entities, &patrons, &feelings) {
            // For each patron, spawn a thought bubble that shows what
            // they are thinking.

            // Does the thought bubble already exist?
            // If so, don't recreate it.
            // TODO

            let mut thought_local = Transform::default();
            thought_local.prepend_translation_y(20.0);

            let thought_bubble = entities.build_entity()
                .with(ThoughtBubble {})
                .with(Parent {
                    entity: patron_entity
                })
                .with(thought_local)
                .build();

            let mut feeling_local = Transform::default();
            local_transform.prepend_translation_y(3.0);
            local_transform.prepend_translation_z(0.01);
            
            let feeling = entities.build_entity()
                .with(feeling)
                .with(Parent {
                    entity: thought_bubble
                })
                .with(feeling_local)
                .build();
        }
    }
}
