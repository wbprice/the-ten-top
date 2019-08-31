use amethyst::{
    assets::{
        Handle
    },
    core::{
        transform::Transform
    },
    ecs::prelude::World,
    prelude::*,
    renderer::{
        SpriteSheet,
        SpriteRender
    }
};

use crate::components::{
    Food,
    Patron
};

pub fn init_patron(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(0.0, 0.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1
    };

    world.create_entity()
        .with(sprite_render)
        .with(Patron {
            satisfaction: 100,
            velocity: [1.0, 0.0],
            craving: Food::Hamburger
        })
        .with(local_transform)
        .build();
}