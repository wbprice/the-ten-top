use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{components::Dish, resources::Dishes};

pub fn init_dish(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.prepend_translation_x(56.0);
    local_transform.prepend_translation_y(80.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 21,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Dish {
            dish: Dishes::HotDog,
        })
        .with(local_transform)
        .build()
}
