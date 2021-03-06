use amethyst::{
    assets::Handle,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    components::{Emotion, Feeling},
    resources::Dishes,
};

pub fn init_feeling(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    parent: Entity,
) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.prepend_translation_y(3.0);
    local_transform.prepend_translation_z(0.01);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 8,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Feeling {
            symbol: Emotion::Craving(Dishes::Hamburger),
        })
        .with(Parent { entity: parent })
        .with(local_transform)
        .build()
}
