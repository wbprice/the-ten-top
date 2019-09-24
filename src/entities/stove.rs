use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    common::{Point},
    components::{Stove}
};

pub fn init_stove(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, location: Point) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(location.x, location.y, 0.1);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 4,
    };

    world
        .create_entity()
        .with(Stove::new())
        .with(sprite_render)
        .with(local_transform)
        .build();
}