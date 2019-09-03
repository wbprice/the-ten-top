use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    entities::{
        init_thought_bubble
    },
    components::{Food, Patron, SimpleAnimation}
};

pub fn init_patron(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(12.0, 24.0, 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let patron = world
        .create_entity()
        .with(sprite_render)
        .with(Patron {
            satisfaction: 100,
            velocity: [25.0, 0.0],
            craving: Food::Hamburger,
        })
        .with(SimpleAnimation::new(1, 6, 0.1))
        .with(local_transform)
        .build();

    init_thought_bubble(world, sprite_sheet_handle, patron);
}
