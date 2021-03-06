use amethyst::{
    assets::Handle,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::ThoughtBubble;

pub fn init_thought_bubble(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    parent: Entity,
) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.prepend_translation_y(20.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 10,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(ThoughtBubble {})
        .with(Parent { entity: parent })
        .with(local_transform)
        .build()
}
