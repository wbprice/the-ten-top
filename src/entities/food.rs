use amethyst::{
    assets::Handle,
    core::transform::{
        Transform,
        Parent
    },
    ecs::prelude::{
        World,
        Entity
    },
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{
    Food,
    Dish,
};

pub fn init_food(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, parent: Entity) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.prepend_translation_y(3.0);
    local_transform.prepend_translation_z(0.1);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 8
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Food {
            dish: Dish::Hamburger
        })
        .with(Parent {
            entity: parent
        })
        .with(local_transform)
        .build()
}