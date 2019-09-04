use amethyst::{
    assets::Handle,
    core::transform::Transform,
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
    Parent
};

pub fn init_food(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, parent: Entity) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(12.0, 52.0, 0.5);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 7
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