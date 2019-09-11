use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    components::{Food, Patron, SimpleAnimation, Velocity},
    entities::{init_food, init_thought_bubble},
};

pub fn init_patron(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, patron: Patron) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(12.0, 24.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let patron = world
        .create_entity()
        .with(patron)
        .with(Velocity { x: 15.0, y: 0.0 })
        .with(sprite_render)
        .with(SimpleAnimation::new(1, 6, 0.1))
        .with(local_transform)
        .build();

    let thought_bubble = init_thought_bubble(world, sprite_sheet_handle.clone(), patron);
    init_food(world, sprite_sheet_handle.clone(), thought_bubble);
}
