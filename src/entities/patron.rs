use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    components::{Emotion, Feeling, Patron, Velocity},
    resources::Dishes,
};

pub fn init_patron(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, patron: Patron) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(12.0, 24.0, 0.2);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 5,
    };

    world
        .create_entity()
        .with(patron)
        .with(sprite_render)
        .with(Feeling {
            symbol: Emotion::Craving(Dishes::HotDog),
        })
        .with(Velocity { x: 15.0, y: 0.0 })
        .with(local_transform)
        .build();
}
