use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    common::{Point},
    components::{
        Plate,
    }
};

pub fn init_plate(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, point: Point) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(point.x, point.y, 0.2);

    world.create_entity()
        .with(Plate {})
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 2
        })
        .with(local_transform)
        .build();
}