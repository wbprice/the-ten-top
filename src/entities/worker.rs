use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Dish, Worker, Emotion, Feeling, Patron, SimpleAnimation, Velocity};

pub fn init_worker(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(44.0, 108.0, 0.2);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Worker {})
        .with(sprite_render)
        .with(local_transform)
        .with(Velocity {
            x: 0.0,
            y: 0.0
        })
        .build();
}