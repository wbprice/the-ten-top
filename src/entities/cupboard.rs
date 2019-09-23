use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Cupboard};

pub fn init_cupboard(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, cupboard: Cupboard) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(12.0, 24.0, 0.2);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 6,
    };

    world
        .create_entity()
        .with(cupboard)
        .with(sprite_render)
        .with(local_transform)
        .build();
}