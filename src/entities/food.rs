use amethyst::{
    assets::Handle,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Dish, Food};

pub fn init_food(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.prepend_translation_y(56.0);
    local_transform.prepend_translation_z(80.00);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 7,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Food {
            dish: Dish::Hamburger,
        })
        .with(local_transform)
        .build()
}
