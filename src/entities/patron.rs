use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    components::{Food, Patron, SimpleAnimation},
    entities::{init_food, init_thought_bubble},
};

pub fn init_patron(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, patron: Patron) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(patron.origin[0], patron.origin[1], 0.0);

    // If horizontal velocity is negative, flip the sprites
    if patron.velocity[0] < 0.0 {
        local_transform.set_rotation_y_axis(3.14);
    }

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let patron = world
        .create_entity()
        .with(patron)
        .with(sprite_render)
        .with(SimpleAnimation::new(1, 6, 0.1))
        .with(local_transform)
        .build();

    let thought_bubble = init_thought_bubble(world, sprite_sheet_handle.clone(), patron);
    init_food(world, sprite_sheet_handle.clone(), thought_bubble);
}